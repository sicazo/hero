import { createFileRoute } from '@tanstack/react-router'
import {Separator} from "@/components/ui/separator.tsx";
import AppearanceForm from "@/components/settings/appearance/appearance_form.tsx";

export const Route = createFileRoute('/settings/appearance')({
  component: Appearance
})


function Appearance() {
  return (
      <div className="space-y-6">
        <div>
          <h3 className="text-lg font-medium">Appearance</h3>
          <p className="text-sm text-muted-foreground">
            Customize the appearance of the app.
          </p>
        </div>
        <Separator/>
        <AppearanceForm/>
      </div>

  )
}