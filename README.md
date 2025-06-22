# Nomino

> Cross-platform desktop application for bulk folder renaming using Excel data with Azerbaijani alphabet support

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-1.5-blue.svg)](https://tauri.app/)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-2.0-orange.svg)](https://kit.svelte.dev/)

## Overview

Nomino is a powerful desktop application built with Tauri and SvelteKit that enables bulk folder renaming operations using data from Excel files. It features special support for Azerbaijani alphabet sorting and provides a modern, intuitive interface for file management tasks.

## Features

### 🗂️ Folder Management
- **Bulk Folder Renaming**: Rename multiple folders simultaneously
- **Excel Integration**: Import new folder names from Excel files (.xlsx, .xls)
- **Flexible Column Selection**: Choose any column (A-Z) and starting row
- **Preview Mode**: See changes before applying them

### 🔤 Sorting & Localization
- **Azerbaijani Alphabet Support**: Proper sorting with Azerbaijani character order
- **Multiple Sorting Options**: Default, numeric, and Azerbaijani alphabet sorting
- **Windows Explorer Compatibility**: Maintains consistent ordering with file explorer

### 🎯 User Experience
- **Real-time Progress**: Live progress tracking with detailed logging
- **Process Control**: Start, pause, stop, and reset operations
- **Error Handling**: Comprehensive error reporting and recovery
- **Modern UI**: Clean, responsive interface with dark theme

## Screenshots

*[Add screenshots of your application here]*

## Installation

### Prerequisites
- **Node.js** 18.0.0 or higher
- **Rust** 1.70.0 or higher
- **npm** 8.0.0 or higher

### Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/nomino.git
   cd nomino
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Generate application icons**
   ```bash
   npm run icons:generate
   ```

4. **Start development server**
   ```bash
   npm run tauri:dev
   ```

### Building for Production

1. **Build the application**
   ```bash
   npm run tauri:build
   ```

2. **Find the executable**
   - Windows: `src-tauri/target/release/nomino.exe`
   - macOS: `src-tauri/target/release/bundle/dmg/`
   - Linux: `src-tauri/target/release/bundle/appimage/`

## Usage

### Basic Workflow

1. **Select Source Folder**: Choose the folder containing subfolders to rename
2. **Choose Sorting Method**: Select default, numeric, or Azerbaijani alphabet sorting
3. **Set Destination**: Pick where renamed folders should be created
4. **Import Excel File**: Load Excel file with new folder names
5. **Configure Excel Settings**: Set starting row and column for names
6. **Preview Changes**: Review the rename operations
7. **Execute**: Start the renaming process with real-time progress

### Excel File Format

Your Excel file should contain folder names in a single column:

| A | B | C |
|---|---|---|
| New Folder Name 1 | | |
| New Folder Name 2 | | |
| New Folder Name 3 | | |

- **Starting Row**: Row number where names begin (e.g., 1, 2, 3...)
- **Column**: Letter of the column containing names (A, B, C, etc.)

### Azerbaijani Alphabet Sorting

The application supports proper Azerbaijani alphabet ordering:
```
a, b, c, ç, d, e, ə, f, g, ğ, h, x, ı, i, j, k, q, l, m, n, o, ö, p, r, s, ş, t, u, ü, v, w, y, z
```

## Project Structure

```
nomino/
├── src/                    # Frontend source code
│   ├── lib/               # Shared components and utilities
│   │   ├── components/    # Svelte components
│   │   └── assets/        # Static assets
│   └── routes/            # SvelteKit routes
│       ├── +layout.svelte # Main layout
│       └── +page.svelte   # Folder renaming page
├── src-tauri/             # Rust backend
│   ├── src/               # Rust source code
│   │   ├── main.rs        # Main application entry
│   │   └── commands.rs    # Tauri commands
│   ├── icons/             # Application icons
│   └── Cargo.toml         # Rust dependencies
├── assets/                # Source assets
│   └── logo.png           # Application logo
├── scripts/               # Build and utility scripts
│   └── generate-icons.js  # Icon generation script
├── static/                # Static web assets
└── package.json           # Node.js dependencies
```

## Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run tauri:dev` - Start Tauri development mode
- `npm run tauri:build` - Build Tauri application
- `npm run icons:generate` - Generate application icons
- `npm run check` - Run type checking
- `npm run clean` - Clean build artifacts

### Technology Stack

- **Frontend**: SvelteKit, TypeScript, Tailwind CSS
- **Backend**: Rust, Tauri
- **Icons**: Lucide Svelte
- **Excel Processing**: Calamine (Rust)
- **Build Tools**: Vite, PostCSS

### Code Style

The project follows these conventions:
- **TypeScript** for type safety
- **ESLint** for code linting
- **Prettier** for code formatting
- **JSDoc** comments for documentation
- **Conventional Commits** for commit messages

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow the existing code style
- Add tests for new features
- Update documentation as needed
- Ensure all checks pass before submitting

## Troubleshooting

### Common Issues

**Issue**: Application won't start
- **Solution**: Ensure all dependencies are installed and Rust toolchain is up to date

**Issue**: Icons not displaying
- **Solution**: Run `npm run icons:generate` to regenerate icons

**Issue**: Excel file not reading correctly
- **Solution**: Verify file format is .xlsx or .xls and check column/row settings

### Getting Help

- Check the [Issues](https://github.com/yourusername/nomino/issues) page
- Create a new issue with detailed information
- Include system information and error messages

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing desktop app framework
- [SvelteKit](https://kit.svelte.dev/) - For the excellent web framework
- [Lucide](https://lucide.dev/) - For the beautiful icons
- [Tailwind CSS](https://tailwindcss.com/) - For the utility-first CSS framework

---

**Made with ❤️ for efficient folder management** 