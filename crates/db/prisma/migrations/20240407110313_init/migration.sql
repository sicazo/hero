-- CreateTable
CREATE TABLE "Settings" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "nav_open" BOOLEAN NOT NULL DEFAULT false,
    "theme" TEXT NOT NULL DEFAULT 'light',
    "notifications_enabled" BOOLEAN NOT NULL DEFAULT false,
    "toast_rich_colors" BOOLEAN NOT NULL DEFAULT true,
    "notification_file_changes" BOOLEAN NOT NULL DEFAULT false,
    "notification_finished_translation" BOOLEAN NOT NULL DEFAULT false,
    "finished_scan" BOOLEAN NOT NULL DEFAULT false,
    "translate_new_strings" BOOLEAN NOT NULL DEFAULT false,
    "translate_updated_strings" BOOLEAN NOT NULL DEFAULT false,
    "default_language" TEXT NOT NULL DEFAULT 'en-GB',
    "translation_command" TEXT NOT NULL DEFAULT '',
    "home_default_size_nav" INTEGER NOT NULL DEFAULT 4,
    "home_default_size_home" INTEGER NOT NULL DEFAULT 96,
    "home_nav_collapsed" BOOLEAN NOT NULL DEFAULT true,
    "home_collapsed_nav_size" INTEGER NOT NULL DEFAULT 4
);

-- CreateTable
CREATE TABLE "Location" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "tag" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "is_favourite" BOOLEAN NOT NULL DEFAULT false,
    "num_of_keys" INTEGER NOT NULL,
    "num_of_untranslated_keys" INTEGER NOT NULL,
    "added_at" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Location_name_key" ON "Location"("name");

-- CreateIndex
CREATE UNIQUE INDEX "Location_path_key" ON "Location"("path");
