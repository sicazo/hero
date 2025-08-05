"use client"

import { useState, useEffect, useMemo, useCallback } from "react"
import {
  Search,
  Plus,
  Trash2,
  Edit,
  MoreHorizontal,
  X,
  Check,
  ChevronLeft,
  ChevronRight,
  ChevronsLeft,
  ChevronsRight,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Checkbox } from "@/components/ui/checkbox"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { Badge } from "@/components/ui/badge"
import { useLocationStore } from "@/lib/stores/location_store"
import { useTranslationStore } from "@/lib/stores/translation_store"
import { rspc } from "@/lib/rspc"
import { toast } from "sonner"

interface TranslationKey {
  id: string
  typescriptKey: string
  jsonKey: string
  translations: Record<string, string>
  inUse?: boolean
}

export default function TranslationManager() {

  const { locations, last_selected_location, setLastSelectedLocation } = useLocationStore()
  const { translation_entries, languages, setTranslationEntries, removeKeysFromTranslationEntries } = useTranslationStore()

  
  // Local state
  const [filterText, setFilterText] = useState("")
  const [selectedKeys, setSelectedKeys] = useState<string[]>([])
  const [editingKey, setEditingKey] = useState<TranslationKey | null>(null)
  const [currentPage, setCurrentPage] = useState(1)
  const [itemsPerPage, setItemsPerPage] = useState(10)
  const [languageSearch, setLanguageSearch] = useState("")
  
  // Reference for the table container to calculate available height
  const [tableContainerRef, setTableContainerRef] = useState<HTMLDivElement | null>(null)
  
  const [showAddKeyModal, setShowAddKeyModal] = useState(false)
  const [showAddLanguageModal, setShowAddLanguageModal] = useState(false)
  const [newKey, setNewKey] = useState({ typescriptKey: "", jsonKey: "", translations: { "en-GB": "" } })
  const [newLanguageCode, setNewLanguageCode] = useState("")

  // RSPC mutations
  const getTranslationsMutation = rspc.useMutation("translations.get_translations")
  const addKeyMutation = rspc.useMutation("translations.add_key")
  const removeKeysMutation = rspc.useMutation("translations.remove_keys")
  const updateKeysMutation = rspc.useMutation("translations.update_keys")

  // OPTIMIZATION 0: Memoize translation keys conversion
  // This prevents unnecessary recalculations when the component re-renders
  // but translation_entries haven't changed
  const translationKeys = useMemo(() => 
    translation_entries.map((entry) => ({
      id: entry.key || "",
      typescriptKey: entry.key || "",
      jsonKey: entry.value || "",
      translations: entry.translations || {},
      inUse: entry.in_use
    }))
  , [translation_entries])


  const loadTranslations = useCallback(async (locationPath: string) => {
    try {
      const translations = await getTranslationsMutation.mutateAsync(locationPath)
      setTranslationEntries(translations)
      setCurrentPage(1)
      setSelectedKeys([])
    } catch (error) {
      toast.error("Failed to load translations")
    }
  }, [getTranslationsMutation, setTranslationEntries])
  

  useEffect(() => {
    // Function to calculate and set the optimal number of rows
    const calculateOptimalRows = () => {
      if (tableContainerRef) {
        // Get the available height of the table container
        const availableHeight = tableContainerRef.clientHeight
        
        // Approximate height of a table row (including borders)
        const rowHeight = 53 // 53px is the approximate height of each row with padding and borders
        
        // Calculate how many rows can fit in the available space
        // Subtract header height (53px) and leave some margin at the bottom (20px)
        const optimalRows = Math.max(1, Math.floor((availableHeight - 53 - 20) / rowHeight)) - 1
        
        // Update itemsPerPage if the calculated value is different
        if (optimalRows !== itemsPerPage) {
          setItemsPerPage(optimalRows)
        }
      }
    }
    
    // Calculate on mount and when container reference changes
    calculateOptimalRows()
    
    // Add resize event listener to recalculate on window resize
    window.addEventListener('resize', calculateOptimalRows)
    
    // Clean up event listener on component unmount
    return () => {
      window.removeEventListener('resize', calculateOptimalRows)
    }
  }, [tableContainerRef, itemsPerPage])


  const filteredTranslations = useMemo(() => 
    translationKeys.filter(
      (translation) =>
        translation.typescriptKey.toLowerCase().includes(filterText.toLowerCase()) ||
        translation.jsonKey.toLowerCase().includes(filterText.toLowerCase()) ||
        Object.values(translation.translations).some((value) => 
          value.toLowerCase().includes(filterText.toLowerCase())
        )
    )
  , [translationKeys, filterText])


  const paginationData = useMemo(() => {
    const totalPages = Math.ceil(filteredTranslations.length / itemsPerPage)
    const startIndex = (currentPage - 1) * itemsPerPage
    const paginatedTranslations = filteredTranslations.slice(startIndex, startIndex + itemsPerPage)
    
    return {
      totalPages,
      startIndex,
      paginatedTranslations
    }
  }, [filteredTranslations, itemsPerPage, currentPage])
  
  const { totalPages, paginatedTranslations } = paginationData

  // Handle selection
  const handleSelectAll = (checked: boolean) => {
    if (checked) {
      setSelectedKeys(paginatedTranslations.map((t) => t.id))
    } else {
      setSelectedKeys([])
    }
  }

  const handleSelectKey = (keyId: string, checked: boolean) => {
    if (checked) {
      setSelectedKeys([...selectedKeys, keyId])
    } else {
      setSelectedKeys(selectedKeys.filter((id) => id !== keyId))
    }
  }

  // Handle editing
  const handleEditKey = (translation: TranslationKey) => {
    setEditingKey({ ...translation })
  }

  const handleSaveEdit = async () => {
    if (editingKey && last_selected_location) {
      try {
        // Find the original entry to compare changes
        const originalEntry = translationKeys.find(entry => entry.id === editingKey.id)
        if (!originalEntry) return
        
        // Determine which translations have changed
        const changedTranslations: Record<string, string> = {}
        Object.entries(editingKey.translations).forEach(([lang, value]) => {
          if (originalEntry.translations[lang] !== value) {
            changedTranslations[lang] = value
          }
        })
        
        // Only update if there are changes
        if (Object.keys(changedTranslations).length > 0) {
          await updateKeysMutation.mutateAsync({
            path: last_selected_location.path,
            key: {
              ts_key: editingKey.typescriptKey,
              json_key: editingKey.jsonKey,
              translation_values: changedTranslations
            }
          })
          
          // Refresh translations
          const translations = await getTranslationsMutation.mutateAsync(last_selected_location.path)
          setTranslationEntries(translations)
          
          toast.success("Translation updated successfully")
        }
        
        setEditingKey(null)
      } catch (error) {
        toast.error("Failed to update translation")
      }
    }
  }

  // Handle deletion
  const handleDeleteSelected = async () => {
    if (selectedKeys.length === 0 || !last_selected_location) return
    
    try {
      const tsKeys = selectedKeys
      const jsonKeys = selectedKeys.map(key => {
        const entry = translationKeys.find(t => t.id === key)
        return entry ? entry.jsonKey : ""
      }).filter(Boolean)
      
      await removeKeysMutation.mutateAsync({
        path: last_selected_location.path,
        ts_key: tsKeys,
        json_key: jsonKeys
      })
      
      // Update local state
      removeKeysFromTranslationEntries(tsKeys)
      setSelectedKeys([])
      
      toast.success("Keys deleted successfully")
    } catch (error) {
      toast.error("Failed to delete keys")
    }
  }

  // Handle translation value updates
  const updateTranslationValue = (locale: string, value: string) => {
    if (editingKey) {
      setEditingKey({
        ...editingKey,
        translations: {
          ...editingKey.translations,
          [locale]: value,
        },
      })
    }
  }

  // Handle adding/removing languages
  const handleAddLanguage = () => {
    setShowAddLanguageModal(true)
  }

  const handleSaveNewLanguage = () => {
    if (editingKey && newLanguageCode && !editingKey.translations[newLanguageCode]) {
      setEditingKey({
        ...editingKey,
        translations: {
          ...editingKey.translations,
          [newLanguageCode]: "",
        },
      })
      setNewLanguageCode("")
      setShowAddLanguageModal(false)
    }
  }

  const removeLocale = (locale: string) => {
    if (editingKey) {
      // Create a new translations object without the specified locale
      const newTranslations = { ...editingKey.translations };
      delete newTranslations[locale];
      
      setEditingKey({
        ...editingKey,
        translations: newTranslations,
      })
    }
  }

  // Handle adding new keys
  const handleAddKey = () => {
    setShowAddKeyModal(true)
  }

  const handleSaveNewKey = async () => {
    if (!newKey.typescriptKey || !newKey.jsonKey || !last_selected_location) return
    
    try {
      await addKeyMutation.mutateAsync({
        path: last_selected_location.path,
        ts_key: newKey.typescriptKey,
        json_key: newKey.jsonKey,
        value: newKey.translations["en-GB"] || ""
      })
      
      // Refresh translations
      const translations = await getTranslationsMutation.mutateAsync(last_selected_location.path)
      setTranslationEntries(translations)
      
      setNewKey({ typescriptKey: "", jsonKey: "", translations: { "en-GB": "" } })
      setShowAddKeyModal(false)
      
      toast.success("New key added successfully")
    } catch (error) {
      toast.error("Failed to add new key")
    }
  }

  // Handle location/project selection
  const handleLocationChange = (locationName: string) => {
    const location = locations?.find(loc => loc.name === locationName)
    if (location) {
      setLastSelectedLocation(location)
    }
  }

  useEffect(() => {
    if (last_selected_location?.path) {
      loadTranslations(last_selected_location.path)
    }

  }, [last_selected_location])

  return (
    <div className="flex h-screen bg-gray-50">
      {/* Sidebar */}
      <div className="w-64 bg-white border-r border-gray-200 p-4">
        <div className="space-y-4">
          <div>
            <Label htmlFor="project-select" className="text-sm font-medium text-gray-700">
              Project
            </Label>
            <Select 
              value={last_selected_location?.name || ""} 
              onValueChange={handleLocationChange}
            >
              <SelectTrigger id="project-select" className="mt-1">
                <SelectValue placeholder="Select a project" />
              </SelectTrigger>
              <SelectContent>
                {locations?.map(location => (
                  <SelectItem key={location.name} value={location.name}>
                    {location.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <h3 className="text-sm font-medium text-gray-700">Quick Stats</h3>
            <div className="space-y-1 text-sm text-gray-600">
              <div>Total Keys: {translationKeys.length}</div>
              <div>Languages: {languages.length}</div>
              <div>Selected: {selectedKeys.length}</div>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col">
        {/* Header */}
        <div className="bg-white border-b border-gray-200 p-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
                <Input
                  placeholder="Filter keys..."
                  value={filterText}
                  onChange={(e) => setFilterText(e.target.value)}
                  className="pl-10 w-80"
                />
              </div>
            </div>

            <div className="flex items-center space-x-2">
              <Button variant="outline" size="sm" onClick={handleAddKey}>
                <Plus className="w-4 h-4 mr-2" />
                Add
              </Button>
              <Button
                variant="destructive"
                size="sm"
                disabled={selectedKeys.length === 0}
                onClick={handleDeleteSelected}
              >
                <Trash2 className="w-4 h-4 mr-2" />
                Delete Selected Keys
              </Button>
            </div>
          </div>
        </div>

        {/* Table */}
        <div 
          className="flex-1 overflow-auto"
          ref={setTableContainerRef}
        >
          <table className="w-full">
            <thead className="bg-gray-50 border-b border-gray-200 sticky top-0">
              <tr>
                <th className="w-12 px-4 py-3 text-left">
                  <Checkbox
                    checked={selectedKeys.length === paginatedTranslations.length && paginatedTranslations.length > 0}
                    onCheckedChange={handleSelectAll}
                  />
                </th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-700">Typescript Key</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-700">Json Key</th>
                <th className="px-4 py-3 text-left text-sm font-medium text-gray-700">en-GB</th>
                <th className="w-12 px-4 py-3"></th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {paginatedTranslations.map((translation) => (
                <tr key={translation.id} className="hover:bg-gray-50">
                  <td className="px-4 py-3">
                    <Checkbox
                      checked={selectedKeys.includes(translation.id)}
                      onCheckedChange={(checked) => handleSelectKey(translation.id, checked as boolean)}
                    />
                  </td>
                  <td className="px-4 py-3 text-sm text-gray-900 font-mono">{translation.typescriptKey}</td>
                  <td className="px-4 py-3 text-sm text-gray-900 font-mono">{translation.jsonKey}</td>
                  <td className="px-4 py-3 text-sm text-gray-900">{translation.translations["en-GB"] || "-"}</td>
                  <td className="px-4 py-3">
                    <DropdownMenu>
                      <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="sm">
                          <MoreHorizontal className="w-4 h-4" />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent align="end">
                        <DropdownMenuItem onClick={() => handleEditKey(translation)}>
                          <Edit className="w-4 h-4 mr-2" />
                          Edit
                        </DropdownMenuItem>
                        <DropdownMenuItem 
                          className="text-red-600"
                          onClick={() => {
                            setSelectedKeys([translation.id])
                            handleDeleteSelected()
                          }}
                        >
                          <Trash2 className="w-4 h-4 mr-2" />
                          Delete
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </td>
                </tr>
              ))}
              {paginatedTranslations.length === 0 && (
                <tr>
                  <td colSpan={5} className="px-4 py-8 text-center text-sm text-gray-500">
                    {filterText ? "No translations found matching your filter" : "No translations available"}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>

        {/* Pagination */}
        <div className="bg-white border-t border-gray-200 px-4 py-3 flex items-center justify-between">
          <div className="text-sm text-gray-700">
            {selectedKeys.length} of {filteredTranslations.length} row(s) selected.
          </div>
          <div className="flex items-center space-x-2">
            <span className="text-sm text-gray-700">
              Page {currentPage} of {totalPages || 1}
            </span>
            <div className="flex items-center space-x-1">
              <Button variant="outline" size="sm" onClick={() => setCurrentPage(1)} disabled={currentPage === 1}>
                <ChevronsLeft className="w-4 h-4" />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(currentPage - 1)}
                disabled={currentPage === 1}
              >
                <ChevronLeft className="w-4 h-4" />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(currentPage + 1)}
                disabled={currentPage === totalPages || totalPages === 0}
              >
                <ChevronRight className="w-4 h-4" />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(totalPages)}
                disabled={currentPage === totalPages || totalPages === 0}
              >
                <ChevronsRight className="w-4 h-4" />
              </Button>
            </div>
          </div>
        </div>
      </div>

      {/* Edit Dialog */}
      <Dialog
        open={!!editingKey}
        onOpenChange={(open) => {
          if (!open) {
            setEditingKey(null)
            setLanguageSearch("")
          }
        }}
      >
        <DialogContent className="max-w-7xl max-h-[90vh] overflow-auto w-[90vw]">
          <DialogHeader>
            <DialogTitle className="font-mono text-lg">Edit {editingKey?.typescriptKey}</DialogTitle>
          </DialogHeader>

          {editingKey && (
            <div className="space-y-6">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label htmlFor="ts-key">TS Key</Label>
                  <Input
                    id="ts-key"
                    value={editingKey.typescriptKey}
                    className="font-mono"
                    disabled
                  />
                </div>
                <div>
                  <Label htmlFor="json-key">Json Key</Label>
                  <Input
                    id="json-key"
                    value={editingKey.jsonKey}
                    className="font-mono"
                    disabled
                  />
                </div>
              </div>

              <div>
                <div className="flex items-center justify-between mb-4">
                  <Label className="text-base font-medium">Translations</Label>
                  <div className="flex items-center space-x-2">
                    <div className="relative">
                      <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
                      <Input
                        placeholder="Search languages..."
                        value={languageSearch}
                        onChange={(e) => setLanguageSearch(e.target.value)}
                        className="pl-10 w-48"
                      />
                    </div>
                    <Button variant="outline" size="sm" onClick={handleAddLanguage}>
                      <Plus className="w-4 h-4 mr-2" />
                      Add Language
                    </Button>
                  </div>
                </div>

                <div className="space-y-4 max-h-96 overflow-auto">
                  {Object.entries(editingKey.translations)
                    .filter(([locale]) => locale.toLowerCase().includes(languageSearch.toLowerCase()))
                    .map(([locale, value]) => (
                      <div key={locale} className="flex items-start space-x-2">
                        <Badge variant="secondary" className="mt-2 min-w-[80px] justify-center">
                          {locale}
                        </Badge>
                        <Textarea
                          value={value}
                          onChange={(e) => updateTranslationValue(locale, e.target.value)}
                          className="flex-1 min-h-[60px]"
                          placeholder={`Translation for ${locale}`}
                        />
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => removeLocale(locale)}
                          className="mt-2 text-red-600 hover:text-red-700"
                        >
                          <X className="w-4 h-4" />
                        </Button>
                      </div>
                    ))}
                  {Object.entries(editingKey.translations).filter(([locale]) =>
                    locale.toLowerCase().includes(languageSearch.toLowerCase()),
                  ).length === 0 &&
                    languageSearch && (
                      <div className="text-center py-8 text-gray-500">
                        No languages found matching "{languageSearch}"
                      </div>
                    )}
                </div>
              </div>

              <div className="flex justify-end space-x-2 pt-4 border-t">
                <Button variant="outline" onClick={() => setEditingKey(null)}>
                  Cancel
                </Button>
                <Button onClick={handleSaveEdit}>
                  <Check className="w-4 h-4 mr-2" />
                  Submit
                </Button>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>

      {/* Add Key Dialog */}
      <Dialog open={showAddKeyModal} onOpenChange={setShowAddKeyModal}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>Add New Translation Key</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label htmlFor="new-ts-key">TypeScript Key</Label>
              <Input
                id="new-ts-key"
                value={newKey.typescriptKey}
                onChange={(e) => setNewKey({ ...newKey, typescriptKey: e.target.value })}
                placeholder="e.g., UserProfileTitle"
                className="font-mono"
              />
            </div>
            <div>
              <Label htmlFor="new-json-key">JSON Key</Label>
              <Input
                id="new-json-key"
                value={newKey.jsonKey}
                onChange={(e) => setNewKey({ ...newKey, jsonKey: e.target.value })}
                placeholder="e.g., UserProfileTitle"
                className="font-mono"
              />
            </div>
            <div>
              <Label htmlFor="new-translation">Initial Translation (en-GB)</Label>
              <Textarea
                id="new-translation"
                value={newKey.translations["en-GB"]}
                onChange={(e) =>
                  setNewKey({
                    ...newKey,
                    translations: { ...newKey.translations, "en-GB": e.target.value },
                  })
                }
                placeholder="Enter the English translation"
                className="min-h-[80px]"
              />
            </div>
            <div className="flex justify-end space-x-2 pt-4 border-t">
              <Button variant="outline" onClick={() => setShowAddKeyModal(false)}>
                Cancel
              </Button>
              <Button onClick={handleSaveNewKey} disabled={!newKey.typescriptKey || !newKey.jsonKey}>
                <Plus className="w-4 h-4 mr-2" />
                Add Key
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Add Language Dialog */}
      <Dialog open={showAddLanguageModal} onOpenChange={setShowAddLanguageModal}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Add New Language</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label htmlFor="new-language-code">Language Code</Label>
              <Input
                id="new-language-code"
                value={newLanguageCode}
                onChange={(e) => setNewLanguageCode(e.target.value)}
                placeholder="e.g., fr-FR, es-ES, it-IT"
                className="font-mono"
              />
              <p className="text-sm text-gray-500 mt-1">Use standard locale codes like en-US, de-DE, fr-FR</p>
            </div>
            <div className="flex justify-end space-x-2 pt-4 border-t">
              <Button variant="outline" onClick={() => setShowAddLanguageModal(false)}>
                Cancel
              </Button>
              <Button
                onClick={handleSaveNewLanguage}

              >
                <Plus className="w-4 h-4 mr-2" />
                Add Language
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}