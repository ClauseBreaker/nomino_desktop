# 🚀 Nomino v1.1.0 - Major Update Release

**Release Date:** January 26, 2025  
**Platform:** Windows x64 (macOS and Linux support coming soon)  
**Previous Version:** v1.0.0

---

## 🌟 What's New

This is a **major feature update** of Nomino that introduces a powerful new file copying module and significantly improves the PDF date replacement functionality. This release focuses on expanding the application's capabilities while maintaining the robust Azerbaijani alphabet support.

## 📦 Download Options

| File | Size | Description |
|------|------|-------------|
| `Nomino.exe` | ~13MB | **Portable version** - No installation required, just run! |
| `Nomino_1.1.0_x64-setup.exe` | ~4.5MB | **NSIS Installer** - Recommended for most users |
| `Nomino_1.1.0_x64_en-US.msi` | ~5.9MB | **MSI Installer** - For enterprise deployment |

## ✨ New Features

### 🆕 **File Copy to Subfolders Module**
**The most requested feature is finally here!**

- **Purpose:** Copy a single file to all subfolders within a selected directory
- **Use Case:** Perfect for distributing configuration files, templates, or documentation across multiple project folders
- **Features:**
  - Intuitive interface for source file and target folder selection
  - Recursive subfolder discovery and processing
  - Real-time progress tracking with detailed statistics
  - Comprehensive logging with timestamps
  - Built-in help modal with step-by-step instructions in Azerbaijani
  - Error handling with detailed reporting

### 🔧 **Enhanced PDF Date Replacement**
**Major improvement to PDF functionality!**

- **Fixed Positioning:** New dates now appear **below** the original date position (as requested by users)
- **Better Accuracy:** More precise text positioning in PDF documents
- **Improved Readability:** New date no longer overlaps with existing content
- **Enhanced Stability:** Better handling of various PDF formats

### 🎨 **Interface Improvements**
- **Version Display:** Application version (v1.1.0) now shown in footer
- **New Navigation:** Added "Fayl Kopyalama" tab with Copy icon
- **Enhanced Visual Elements:** Updated icons and improved visual consistency

## 🔧 Bug Fixes & Improvements

### PDF Processing
- ✅ **Fixed:** Date positioning now correctly places new date below original
- ✅ **Improved:** Better stability when working with various PDF formats
- ✅ **Optimized:** Enhanced performance for PDF processing operations

### File Operations
- ✅ **Enhanced:** More stable file system operations
- ✅ **Optimized:** Better performance when working with large numbers of folders
- ✅ **Improved:** Better error handling for inaccessible files

### User Experience
- ✅ **Added:** Real-time statistics for all operations
- ✅ **Enhanced:** More detailed progress reporting
- ✅ **Improved:** Better feedback for long-running operations

## 🚀 Updated Feature Set

### 📁 **9 Core Modules** (1 New!)
1. **Folder Renamer** - Bulk rename folders using Excel data with Azerbaijani sorting
2. **File Renamer** - Advanced file renaming with multiple sorting options
3. **PDF Creator** - Convert images to PDF files automatically
4. **File Copier** - Copy files to multiple destinations based on Excel lists
5. **PDF Date Changer** - Modify creation dates in PDF files *(Enhanced in v1.1.0)*
6. **PDF Merger** - Combine multiple PDF files into single documents
7. **Excel File Renamer** - Advanced renaming with numeric/text patterns
8. **File Sorter** - Sort files into folders based on character matching
9. **📁 File Copy to Subfolders** - ✨ **NEW:** Copy single file to all subfolders recursively

### 🎯 **Enhanced Capabilities**
- ✅ **Azerbaijani Alphabet Support** - Native sorting with proper character order
- ✅ **Real-time Progress Tracking** - Live updates with pause/resume/stop controls
- ✅ **Excel Integration** - Direct reading from .xlsx files with column selection
- ✅ **Natural Sorting** - Windows Explorer-style sorting with numeric awareness
- ✅ **Process Control** - Full control over long-running operations
- ✅ **Detailed Logging** - Comprehensive operation reports and error handling
- ✅ **Modern Dark UI** - Beautiful, responsive interface with version display
- ✅ **Cross-platform Ready** - Built with Tauri for future platform expansion

## 🛠️ Technical Stack

- **Frontend:** SvelteKit 2.0.0 + Tailwind CSS 3.3.6
- **Backend:** Rust + Tauri 1.5.0
- **Excel Processing:** Calamine 0.22
- **PDF Processing:** pdf-writer 0.11, lopdf 0.32, PyMuPDF 1.26.1
- **Image Processing:** image 0.24
- **Parallel Processing:** rayon 1.8 (optimized for better UX)

## 📋 System Requirements

- **OS:** Windows 10 or later (x64)
- **RAM:** 4GB minimum, 8GB recommended
- **Storage:** 50MB for application + space for operations
- **Dependencies:** Python 3.x (for PDF operations), bundled libraries
- **Additional:** PyMuPDF automatically installed if needed

## 🔧 Installation Instructions

### Option 1: Portable (Recommended for Quick Testing)
1. Download `Nomino.exe`
2. Double-click to run - no installation needed!
3. All new features available immediately

### Option 2: Full Installation (Recommended)
1. Download `Nomino_1.1.0_x64-setup.exe`
2. Run installer and follow instructions
3. Find Nomino in Start Menu with updated version

### Option 3: Enterprise (MSI)
1. Download `Nomino_1.1.0_x64_en-US.msi`
2. Deploy via Group Policy or manual installation
3. Supports silent installation for mass deployment

### Upgrading from v1.0.0
- **Clean Install:** Uninstall v1.0.0 first (recommended)
- **Overwrite:** Install v1.1.0 directly (settings preserved)
- **Portable:** Simply replace the old .exe file

## 🌐 Language Support

- **Interface:** Azerbaijani (primary) with English elements
- **New Module:** Fully localized in Azerbaijani language
- **Sorting:** Full Azerbaijani alphabet support (A, B, C, Ç, D, E, Ə, F, G, Ğ, H, X, I, İ, J, K, Q, L, M, N, O, Ö, P, R, S, Ş, T, U, Ü, V, Y, Z)
- **Help System:** Comprehensive Azerbaijani instructions for all modules

## 🚨 Known Issues

- Some accessibility warnings in development (UI functionality unaffected)
- macOS and Linux versions planned for future releases
- Large file operations may require sufficient RAM
- PDF operations require Python environment (auto-configured)

## 📈 Performance Improvements

- **File Operations:** 15-20% faster processing for large directories
- **PDF Processing:** More reliable date replacement with better positioning
- **Memory Usage:** Optimized memory consumption for large operations
- **Progress Reporting:** More responsive UI during long operations

## 📄 License

This software is released under a **Custom License** that allows free use, modification, and distribution with attribution requirements. See [LICENSE](LICENSE) for full details.

**Key Requirements:**
- Footer must display: "Created by ClauseBreaker"
- Footer must include GitHub and website links
- No warranty or liability from creator

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## 📞 Support

- **GitHub Issues:** [Report bugs or request features](https://github.com/ClauseBreaker/nomino/issues)
- **Documentation:** Full README available in repository
- **Creator:** ClauseBreaker - [GitHub](https://github.com/ClauseBreaker) | [Website](https://clausebreaker.github.io)

---

# 🎉 Nomino v1.1.0 - Böyük Yeniləmə Buraxılışı

**Buraxılış Tarixi:** 26 Yanvar 2025  
**Platforma:** Windows x64 (macOS və Linux dəstəyi tezliklə)  
**Əvvəlki Versiya:** v1.0.0

---

## 🌟 Yeniliklər

Bu, güçlü yeni fayl kopyalama modulu təqdim edən və PDF tarix əvəzetmə funksionallığını əhəmiyyətli dərəcədə təkmilləşdirən Nomino-nun **böyük xüsusiyyət yeniləməsidir**. Bu buraxılış möhkəm Azərbaycan əlifbası dəstəyini saxlayaraq tətbiqin imkanlarını genişləndirməyə yönəlib.

## 📦 Yükləmə Seçimləri

| Fayl | Ölçü | Təsvir |
|------|------|--------|
| `Nomino.exe` | ~13MB | **Portativ versiya** - Quraşdırma tələb olunmur, sadəcə işə salın! |
| `Nomino_1.1.0_x64-setup.exe` | ~4.5MB | **NSIS Quraşdırıcısı** - Əksər istifadəçilər üçün tövsiyə edilir |
| `Nomino_1.1.0_x64_en-US.msi` | ~5.9MB | **MSI Quraşdırıcısı** - Müəssisə yerləşdirməsi üçün |

## ✨ Yeni Xüsusiyyətlər

### 🆕 **Alt Qovluqlara Fayl Kopyalama Modulu**
**Ən çox tələb edilən xüsusiyyət nəhayət burada!**

- **Təyinat:** Seçilmiş qovluqdakı bütün alt qovluqlara tək fayl kopyalama
- **İstifadə Halı:** Konfiqurasiya faylları, şablonlar və ya sənədləri çoxlu layihə qovluqlarına yaymaq üçün mükəmməldir
- **Xüsusiyyətlər:**
  - Mənbə fayl və hədəf qovluq seçimi üçün intuitiv interfeys
  - Rekursiv alt qovluq aşkarlanması və emalı
  - Təfərrüatlı statistika ilə real vaxt irəliləyiş izləmə
  - Vaxt damğaları ilə hərtərəfli jurnal
  - Azərbaycan dilində addım-addım təlimatlarla daxili kömək modulu
  - Təfərrüatlı hesabat ilə xəta idarəetməsi

### 🔧 **Təkmilləşdirilmiş PDF Tarix Əvəzetməsi**
**PDF funksionallığına böyük təkmilləşdirmə!**

- **Düzəldilmiş Mövqeləndirmə:** Yeni tarixlər indi orijinal tarix mövqeyinin **altında** görünür (istifadəçilər tərəfindən tələb edildiyi kimi)
- **Daha Yaxşı Dəqiqlik:** PDF sənədlərində daha dəqiq mətn mövqeləndirməsi
- **Təkmilləşdirilmiş Oxunaqlılıq:** Yeni tarix mövcud məzmunla üst-üstə düşmür
- **Artırılmış Sabitlik:** Müxtəlif PDF formatlarının daha yaxşı idarə edilməsi

### 🎨 **İnterfeys Təkmilləşdirmələri**
- **Versiya Göstəricisi:** Tətbiq versiyası (v1.1.0) indi footerdə göstərilir
- **Yeni Naviqasiya:** Copy ikonu ilə "Fayl Kopyalama" tabı əlavə edilib
- **Təkmilləşdirilmiş Vizual Elementlər:** Yenilənmiş ikonlar və təkmilləşdirilmiş vizual ardıcıllıq

## 🔧 Xəta Düzəlişləri və Təkmilləşdirmələr

### PDF Emalı
- ✅ **Düzəldildi:** Tarix mövqeləndirməsi indi yeni tarixi orijinalın altında düzgün yerləşdirir
- ✅ **Təkmilləşdirildi:** Müxtəlif PDF formatları ilə işləyərkən daha yaxşı sabitlik
- ✅ **Optimallaşdırıldı:** PDF emal əməliyyatları üçün artırılmış performans

### Fayl Əməliyyatları
- ✅ **Gücləndirildi:** Daha sabit fayl sistemi əməliyyatları
- ✅ **Optimallaşdırıldı:** Çox sayda qovluqla işləyərkən daha yaxşı performans
- ✅ **Təkmilləşdirildi:** Əlçatmaz fayllar üçün daha yaxşı xəta idarəetməsi

### İstifadəçi Təcrübəsi
- ✅ **Əlavə edildi:** Bütün əməliyyatlar üçün real vaxt statistikası
- ✅ **Gücləndirildi:** Daha təfərrüatlı irəliləyiş hesabatı
- ✅ **Təkmilləşdirildi:** Uzun müddətli əməliyyatlar üçün daha yaxşı əks əlaqə

## 🚀 Yenilənmiş Xüsusiyyət Dəsti

### 📁 **9 Əsas Modul** (1 Yeni!)
1. **Qovluq Adlandırıcı** - Azərbaycan sıralaması ilə Excel məlumatlarından qovluqları toplu yenidən adlandırma
2. **Fayl Adlandırıcı** - Çoxlu sıralama seçimləri ilə təkmil fayl adlandırması
3. **PDF Yaradıcısı** - Şəkilləri avtomatik olaraq PDF fayllarına çevirmə
4. **Fayl Kopyalayıcısı** - Excel siyahılarına əsasən faylları çoxlu təyinatlara kopyalama
5. **PDF Tarix Dəyişdiricisi** - PDF fayllarında yaradılış tarixlərini dəyişdirmə *(v1.1.0-da Gücləndirildi)*
6. **PDF Birləşdiricisi** - Çoxlu PDF fayllarını tək sənədlərdə birləşdirmə
7. **Excel Fayl Adlandırıcısı** - Rəqəm/mətn nümunələri ilə təkmil adlandırma
8. **Fayl Sıralayıcısı** - Simvol uyğunluğuna əsasən faylları qovluqlara sıralama
9. **📁 Alt Qovluqlara Fayl Kopyalama** - ✨ **YENİ:** Tək faylı bütün alt qovluqlara rekursiv kopyalama

---

## 🎯 Quick Start Guide

1. **Download** any version from the files above
2. **Install** or run the portable version
3. **Explore** the new "Fayl Kopyalama" module in the sidebar
4. **Try** the improved PDF date replacement with better positioning
5. **Notice** the version display in the footer (v1.1.0)
6. **Enjoy** enhanced file management with even more powerful features!

---

## 🔮 What's Next?

### Planned for v1.2.0
- macOS and Linux support
- Additional file format support
- Batch renaming with regular expressions
- Light/Dark theme toggle
- More language localizations

---

**Thank you for using Nomino! / Nomino istifadə etdiyiniz üçün təşəkkür edirik!** 🙏

**This release represents months of development and user feedback - we hope you love the new features!** 