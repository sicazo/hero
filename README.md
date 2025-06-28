# Hero - Translation Management Tool

![Hero](src/src-tauri/icons/128x128.png)

## Overview

Hero is a powerful desktop application designed to manage translations in large full-stack software projects. Specifically built for React and C# applications, it simplifies the process of adding, updating, and deleting translations across your entire codebase. Built with Tauri, React, and Rust, Hero combines performance and security with a flexible and intuitive user experience.

## Features

- **Translation Management**: Easily add, update, and delete translations across your codebase
- **Multi-Project Support**: Handle translations for React frontend and C# backend components
- **Search & Navigation**: Quickly find and navigate to translation keys
- **Consistency Checking**: Identify missing or inconsistent translations
- **Automated Updates**: Automatically update translation files across your project
- **Version Control Integration**: Track changes to translation files
- **Cross-platform**: Available for Windows, macOS, and Linux
- **Modern UI**: Intuitive interface built with React and Tailwind CSS
- **Auto-updates**: Stay current with the latest features and improvements

## Tech Stack

### Frontend
- React 18.2.0
- TypeScript 5.2.2
- Tailwind CSS 3.4.3
- Radix UI components
- TanStack Router & React Query
- Vite for development and building

### Backend
- Rust 1.75.0
- Tauri 1.6.1
- Prisma Client Rust
- RSPC for type-safe APIs

### Development Tools
- Bun package manager
- Biome for code formatting and linting
- ESLint for additional linting

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (>= 18)
- [Bun](https://bun.sh/) package manager

### Installation

1. Clone the repository
   ```bash
   git clone [repository-url]
   cd hero
   ```

2. Install dependencies
   ```bash
   bun install
   ```

3. Run the development server
   ```bash
   bun run tauri dev
   ```

## Development

### Available Scripts

- `bun run dev` - Start the development server
- `bun run build` - Build the application
- `bun run lint` - Lint the codebase
- `bun run biome` - Format code with Biome
- `bun run preview` - Preview the built application
- `bun run tauri dev` - Start the Tauri development environment

## Project Structure

- `/src` - Frontend React application
  - `/components` - Reusable UI components
  - `/routes` - Application routes
  - `/lib` - Utility functions and hooks
  - `/assets` - Static assets like images
  - `/src-tauri` - Rust backend code

- `/crates` - Additional Rust crates
  - `/server` - Server-related functionality
  - `/db` - Database interaction layer
  - `/translation_handler` - Internationalization support
  - `/local_storage` - Local storage functionality

## Using Hero for Translation Management

### Setting Up Your Project

1. Open Hero and create a new project
2. Configure the paths to your React and C# translation files
3. Hero will scan and index all translation keys

### Managing Translations

- **Add Translations**: Create new translation keys and provide translations for all configured languages
- **Update Translations**: Edit existing translations with inline editing
- **Delete Translations**: Safely remove unused translation keys
- **Find Missing Translations**: Quickly identify and fill gaps in your translation coverage
### Integration with Development Workflow

- Automatically update translation files during development
- Track changes to translations in your version control system

## Building for Production

```bash
bun run tauri build
```

This will create platform-specific packages in the `src-tauri/target/release` directory.

## Why Hero for Translations?

Managing translations in large full-stack applications presents unique challenges:

- **Fragmentation**: Translations spread across different file formats (JSON for React, RESX for C#)
- **Consistency**: Maintaining the same keys and translations across frontend and backend
- **Collaboration**: Enabling non-developers to contribute translations
- **Maintenance**: Identifying and removing unused translation keys

Hero addresses these pain points with a dedicated UI and automated tools designed specifically for translation management in React and C# projects.

## Contact

For questions and support, please open an issue in the GitHub repository.
