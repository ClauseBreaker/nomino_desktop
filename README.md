# Nomino - File Management Desktop Application

<div align="center">

![Nomino Logo](static/logo_new.png)

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/ClauseBreaker/nomino_desktop)
[![License](https://img.shields.io/badge/license-Custom-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/ClauseBreaker/nomino_desktop)
[![Tauri](https://img.shields.io/badge/Tauri-1.5.0-orange.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-4.2.7-red.svg)](https://svelte.dev/)

**A powerful cross-platform desktop application for bulk file and folder management operations with Excel integration and Azerbaijani alphabet support.**

[Download Latest Release](https://github.com/ClauseBreaker/nomino_desktop/releases) • [Documentation](#documentation) • [Report Bug](https://github.com/ClauseBreaker/nomino_desktop/issues)

</div>

## 🌟 Features

### 📁 **Core Modules**
- **Folder Renamer** - Bulk rename folders using Excel data
- **File Renamer** - Bulk rename files using Excel data with advanced filtering
- **PDF Creator** - Convert images to PDF files automatically
- **File Copier** - Copy files to multiple destinations based on Excel lists
- **PDF Date Changer** - Modify creation dates in PDF files
- **PDF Merger** - Combine multiple PDF files into one
- **Excel File Renamer** - Advanced file renaming with numeric/text patterns
- **File Sorter** - Sort files into folders based on character matching

### 🚀 **Key Capabilities**
- **Azerbaijani Alphabet Support** - Native sorting and processing
- **Real-time Progress Tracking** - Live updates with pause/resume/stop controls
- **Excel Integration** - Direct reading from .xlsx files with column selection
- **Natural Sorting** - Windows Explorer-style sorting with numeric awareness
- **Process Control** - Start, pause, resume, and stop operations
- **Detailed Logging** - Comprehensive operation reports and error handling
- **Modern UI** - Dark theme with responsive design
- **Cross-platform** - Windows, macOS, and Linux support

## 📦 Installation

### Quick Start (Portable)
1. Download `nomino.exe` from the [releases page](https://github.com/ClauseBreaker/nomino_desktop/releases)
2. Double-click to run - no installation required!

### Full Installation
1. **NSIS Installer** (Recommended): `Nomino_1.0.0_x64-setup.exe`
   - Automatic installation with Start Menu shortcuts
   - Uninstaller included
   
2. **MSI Installer**: `Nomino_1.0.0_x64_en-US.msi`
   - Enterprise deployment support
   - Group Policy compatible

## 🛠️ Building from Source

### Prerequisites
- **Node.js** >= 18.0.0
- **Rust** >= 1.60.0
- **npm** >= 8.0.0

### Development Setup
```bash
# Clone the repository
git clone https://github.com/ClauseBreaker/nomino_desktop.git
cd nomino_desktop

# Install dependencies
npm install

# Start development server
npm run tauri:dev
```

### Production Build
```bash
# Build for production
npm run tauri:build

# Output files will be in src-tauri/target/release/bundle/
```

## 📖 Documentation

### Module Descriptions

#### 1. **Folder Renamer** (`/`)
Rename folders in bulk using Excel data with Azerbaijani alphabet sorting.

**Features:**
- Excel column mapping
- Natural sorting algorithm
- Progress tracking with pause/resume
- Batch processing with error handling

**Usage:**
1. Select source folder containing subfolders
2. Choose destination folder
3. Select Excel file with new names
4. Configure start row and column
5. Start renaming process

#### 2. **File Renamer** (`/files`)
Advanced file renaming with multiple sorting and filtering options.

**Features:**
- Multiple sorting methods (name, date, size)
- Excel-based renaming
- File type filtering
- Advanced progress tracking

#### 3. **PDF Creator** (`/pdf`)
Convert images to PDF files with automatic processing.

**Features:**
- Batch image-to-PDF conversion
- Subfolder processing
- File cleanup options
- Support for JPG, PNG, BMP, GIF, TIFF, WebP

#### 4. **File Copier** (`/copy`)
Copy files to multiple destinations based on Excel lists.

**Features:**
- Excel-driven file copying
- Bulk operations
- Progress tracking
- Error handling and logging

#### 5. **PDF Date Changer** (`/pdf-date`)
Modify creation dates in PDF files with keyword filtering.

**Features:**
- Batch date modification
- Keyword-based filtering
- Original file backup options
- Date format validation (dd.MM.yyyy)

#### 6. **PDF Merger** (`/pdf-merge`)
Combine multiple PDF files into single documents.

**Features:**
- Recursive folder processing
- Automatic file merging
- Progress tracking
- Error handling

#### 7. **Excel File Renamer** (`/excel-rename`)
Advanced file renaming with numeric and text pattern support.

**Features:**
- Original filename mode
- Numeric pattern mode
- Character limit options
- Start file specification
- Flexible Excel column mapping

#### 8. **File Sorter** (`/file-sorter`)
Sort files into folders based on character prefix matching.

**Features:**
- Character-based matching (1-50 characters)
- Azerbaijani alphabet awareness
- Real-time statistics
- Process control (pause/resume/stop)

### Technical Architecture

#### Frontend (SvelteKit)
- **Framework**: SvelteKit 2.0.0
- **Styling**: Tailwind CSS 3.3.6
- **Icons**: Lucide Svelte 0.294.0
- **Build Tool**: Vite 5.0.3

#### Backend (Rust/Tauri)
- **Runtime**: Tauri 1.5.0
- **Excel Processing**: Calamine 0.22
- **PDF Processing**: pdf-writer 0.11, lopdf 0.32
- **Image Processing**: image 0.24
- **Parallel Processing**: rayon 1.8
- **Async Runtime**: tokio 1.0

#### Key Libraries
- **Natural Sorting**: Custom Azerbaijani alphabet implementation
- **File Operations**: Native Rust std::fs with error handling
- **Process Control**: Atomic operations for thread-safe state management
- **Progress Tracking**: Real-time WebSocket-like communication

## 🌐 Azerbaijani Language Support

Nomino includes comprehensive support for the Azerbaijani alphabet with proper sorting:

**Azerbaijani Alphabet Order:**
A, B, C, Ç, D, E, Ə, F, G, Ğ, H, X, I, İ, J, K, Q, L, M, N, O, Ö, P, R, S, Ş, T, U, Ü, V, Y, Z

**Features:**
- Natural sorting respects Azerbaijani character order
- Case-insensitive comparisons
- Numeric sorting with Azerbaijani text
- Full Unicode support

## 🔧 Configuration

### Excel File Requirements
- Format: `.xlsx` (Excel 2007+)
- Structure: Data in columns with headers
- Encoding: UTF-8 compatible
- Size: No specific limits (memory dependent)

### File System Permissions
- Read access to source directories
- Write access to destination directories
- Temporary file creation permissions

### System Requirements
- **Windows**: 10 or later (x64)
- **macOS**: 10.15 or later
- **Linux**: Ubuntu 18.04+ or equivalent
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 50MB for application, additional for operations

## 🚨 Troubleshooting

### Common Issues

**1. Excel File Not Reading**
- Ensure file is .xlsx format
- Check file permissions
- Verify column letters (A, B, C, etc.)
- Close Excel if file is open

**2. Process Stuck/Hanging**
- Use Stop button to halt operations
- Check destination folder permissions
- Verify sufficient disk space

**3. Character Encoding Issues**
- Ensure Excel file is UTF-8 compatible
- Check Azerbaijani character display
- Verify font support

**4. Performance Issues**
- Close other applications
- Process smaller batches
- Check available RAM

### Error Messages
- **"Qovluq mövcud deyil"** - Folder doesn't exist
- **"Excel faylını açmaq mümkün olmadı"** - Cannot open Excel file
- **"Proses işləmir"** - Process not running
- **"Yanlış sütun hərfi"** - Invalid column letter

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Guidelines
1. Follow Rust and TypeScript best practices
2. Add tests for new features
3. Update documentation
4. Use conventional commit messages
5. Ensure cross-platform compatibility

### Reporting Issues
Please use our [issue tracker](https://github.com/ClauseBreaker/nomino_desktop/issues) with:
- Detailed description
- Steps to reproduce
- System information
- Screenshots if applicable

## 📄 License

This project is licensed under a Custom License that allows free use, modification, and distribution with attribution requirements - see the [LICENSE](LICENSE) file for details.

**Key Requirements:**
- You may use, modify, and distribute this software freely
- The footer must prominently display: "Created by ClauseBreaker"
- The footer must include links to GitHub and website
- No warranty or liability from the creator
- Governed by Azerbaijan and international copyright laws

## 👨‍💻 Author

**ClauseBreaker**
- GitHub: [@ClauseBreaker](https://github.com/ClauseBreaker)
- Website: [clausebreaker.github.io](https://clausebreaker.github.io)

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the excellent desktop app framework
- [SvelteKit](https://kit.svelte.dev/) - For the modern frontend framework
- [Calamine](https://github.com/tafia/calamine) - For Excel file processing
- [Lucide](https://lucide.dev/) - For beautiful icons

## 📊 Statistics

- **Languages**: Rust (70%), TypeScript (25%), CSS (5%)
- **Total Lines**: ~15,000+
- **Modules**: 8 functional modules
- **Commands**: 25+ backend commands
- **File Types Supported**: Excel (.xlsx), PDF, Images (JPG, PNG, etc.)

---

<div align="center">

**Made with ❤️ for the Azerbaijani community**

Faydalı hesab edirsinizsə [⭐ Bu repositoriyanı ulduzlayın](https://github.com/ClauseBreaker/nomino_desktop)!

</div>

---

# Nomino - Fayl İdarəetmə Masaüstü Tətbiqi

<div align="center">

![Nomino Loqosu](static/logo_new.png)

[![Versiya](https://img.shields.io/badge/versiya-1.0.0-blue.svg)](https://github.com/ClauseBreaker/nomino_desktop)
[![Lisenziya](https://img.shields.io/badge/lisenziya-Xüsusi-green.svg)](LICENSE)
[![Platforma](https://img.shields.io/badge/platforma-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/ClauseBreaker/nomino_desktop)
[![Tauri](https://img.shields.io/badge/Tauri-1.5.0-orange.svg)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-4.2.7-red.svg)](https://svelte.dev/)

**Excel inteqrasiyası və Azərbaycan əlifbası dəstəyi ilə toplu fayl və qovluq idarəetmə əməliyyatları üçün güçlü çarpaz platforma masaüstü tətbiqi.**

[Ən Son Buraxılışı Yüklə](https://github.com/ClauseBreaker/nomino_desktop/releases) • [Sənədlər](#sənədlər) • [Xəta Bildir](https://github.com/ClauseBreaker/nomino_desktop/issues)

</div>

## 🌟 Xüsusiyyətlər

### 📁 **Əsas Modullar**
- **Qovluq Adlandırıcı** - Excel məlumatlarından istifadə edərək qovluqları toplu şəkildə yenidən adlandırın
- **Fayl Adlandırıcı** - Təkmil filtrlə Excel məlumatlarından istifadə edərək faylları toplu şəkildə yenidən adlandırın
- **PDF Yaradıcısı** - Şəkilləri avtomatik olaraq PDF fayllarına çevirin
- **Fayl Kopyalayıcısı** - Excel siyahılarına əsasən faylları çoxlu təyinatlara kopyalayın
- **PDF Tarix Dəyişdiricisi** - PDF fayllarında yaradılış tarixlərini dəyişdirin
- **PDF Birləşdiricisi** - Bir neçə PDF faylını birində birləşdirin
- **Excel Fayl Adlandırıcısı** - Rəqəm/mətn nümunələri ilə təkmil fayl adlandırması
- **Fayl Sıralayıcısı** - Simvol uyğunluğuna əsasən faylları qovluqlara sıralayın

### 🚀 **Əsas İmkanlar**
- **Azərbaycan Əlifbası Dəstəyi** - Yerli sıralama və emal
- **Real Vaxt İrəliləyiş İzləmə** - Fasilə/davam/dayandırma nəzarəti ilə canlı yeniləmələr
- **Excel İnteqrasiyası** - Sütun seçimi ilə .xlsx fayllarından birbaşa oxuma
- **Təbii Sıralama** - Rəqəm şüurlu Windows Explorer üslubunda sıralama
- **Proses Nəzarəti** - Əməliyyatları başlatma, fasilə verme, davam etdirmə və dayandırma
- **Təfərrüatlı Jurnal** - Hərtərəfli əməliyyat hesabatları və xəta idarəetməsi
- **Müasir İnterfeys** - Responsiv dizaynla tünd mövzu
- **Çarpaz Platforma** - Windows, macOS və Linux dəstəyi

## 📦 Quraşdırma

### Sürətli Başlanğıc (Portativ)
1. [Buraxılışlar səhifəsindən](https://github.com/ClauseBreaker/nomino_desktop/releases) `nomino.exe` yükləyin
2. İşə salmaq üçün iki dəfə klikləyin - quraşdırma tələb olunmur!

### Tam Quraşdırma
1. **NSIS Quraşdırıcısı** (Tövsiyə edilir): `Nomino_1.0.0_x64-setup.exe`
   - Başlanğıc Menyusu qısayolları ilə avtomatik quraşdırma
   - Qaldırıcı daxildir
   
2. **MSI Quraşdırıcısı**: `Nomino_1.0.0_x64_en-US.msi`
   - Müəssisə yerləşdirmə dəstəyi
   - Qrup Siyasəti uyğun

## 🛠️ Mənbədən Qurma

### Ön Şərtlər
- **Node.js** >= 18.0.0
- **Rust** >= 1.60.0
- **npm** >= 8.0.0

### İnkişaf Quraşdırması
```bash
# Repositoriyanı klonlayın
git clone https://github.com/ClauseBreaker/nomino_desktop.git
cd nomino_desktop

# Asılılıqları quraşdırın
npm install

# İnkişaf serverini başladın
npm run tauri:dev
```

### İstehsal Qurması
```bash
# İstehsal üçün qurun
npm run tauri:build

# Çıxış faylları src-tauri/target/release/bundle/ içində olacaq
```

## 📖 Sənədlər

### Modul Təsvirləri

#### 1. **Qovluq Adlandırıcısı** (`/`)
Azərbaycan əlifbası sıralaması ilə Excel məlumatlarından istifadə edərək qovluqları toplu şəkildə yenidən adlandırın.

**Xüsusiyyətlər:**
- Excel sütun xəritələməsi
- Təbii sıralama alqoritmi
- Fasilə/davam etdirmə ilə irəliləyiş izləmə
- Xəta idarəetməsi ilə toplu emal

**İstifadə:**
1. Alt qovluqları olan mənbə qovluğunu seçin
2. Təyinat qovluğunu seçin
3. Yeni adları olan Excel faylını seçin
4. Başlanğıc sətir və sütunu konfiqurasiya edin
5. Yenidən adlandırma prosesini başladın

#### 2. **Fayl Adlandırıcısı** (`/files`)
Çoxlu sıralama və filtrlə seçimləri ilə təkmil fayl adlandırması.

**Xüsusiyyətlər:**
- Çoxlu sıralama metodları (ad, tarix, ölçü)
- Excel əsaslı yenidən adlandırma
- Fayl növü filtrləməsi
- Təkmil irəliləyiş izləmə

#### 3. **PDF Yaradıcısı** (`/pdf`)
Avtomatik emal ilə şəkilləri PDF fayllarına çevirin.

**Xüsusiyyətlər:**
- Toplu şəkil-PDF çevrilməsi
- Alt qovluq emalı
- Fayl təmizləmə seçimləri
- JPG, PNG, BMP, GIF, TIFF, WebP dəstəyi

#### 4. **Fayl Kopyalayıcısı** (`/copy`)
Excel siyahılarına əsasən faylları çoxlu təyinatlara kopyalayın.

**Xüsusiyyətlər:**
- Excel idarə olunan fayl kopyalama
- Toplu əməliyyatlar
- İrəliləyiş izləmə
- Xəta idarəetməsi və jurnal

#### 5. **PDF Tarix Dəyişdiricisi** (`/pdf-date`)
Açar söz filtrləməsi ilə PDF fayllarında yaradılış tarixlərini dəyişdirin.

**Xüsusiyyətlər:**
- Toplu tarix dəyişikliyi
- Açar söz əsaslı filtrlə
- Orijinal fayl ehtiyat seçimləri
- Tarix format doğrulaması (dd.MM.yyyy)

#### 6. **PDF Birləşdiricisi** (`/pdf-merge`)
Çoxlu PDF fayllarını tək sənədlərdə birləşdirin.

**Xüsusiyyətlər:**
- Rekursiv qovluq emalı
- Avtomatik fayl birləşdirmə
- İrəliləyiş izləmə
- Xəta idarəetməsi

#### 7. **Excel Fayl Adlandırıcısı** (`/excel-rename`)
Rəqəm və mətn nümunə dəstəyi ilə təkmil fayl adlandırması.

**Xüsusiyyətlər:**
- Orijinal fayl adı rejimi
- Rəqəm nümunə rejimi
- Simvol limit seçimləri
- Başlanğıc fayl təyini
- Çevik Excel sütun xəritələməsi

#### 8. **Fayl Sıralayıcısı** (`/file-sorter`)
Simvol prefiks uyğunluğuna əsasən faylları qovluqlara sıralayın.

**Xüsusiyyətlər:**
- Simvol əsaslı uyğunluq (1-50 simvol)
- Azərbaycan əlifbası şüuru
- Real vaxt statistikası
- Proses nəzarəti (fasilə/davam/dayandır)

### Texniki Arxitektura

#### Frontend (SvelteKit)
- **Framework**: SvelteKit 2.0.0
- **Üslublandırma**: Tailwind CSS 3.3.6
- **İkonlar**: Lucide Svelte 0.294.0
- **Qurma Aləti**: Vite 5.0.3

#### Backend (Rust/Tauri)
- **Runtime**: Tauri 1.5.0
- **Excel Emalı**: Calamine 0.22
- **PDF Emalı**: pdf-writer 0.11, lopdf 0.32
- **Şəkil Emalı**: image 0.24
- **Paralel Emal**: rayon 1.8
- **Async Runtime**: tokio 1.0

#### Əsas Kitabxanalar
- **Təbii Sıralama**: Xüsusi Azərbaycan əlifbası tətbiqi
- **Fayl Əməliyyatları**: Xəta idarəetməsi ilə yerli Rust std::fs
- **Proses Nəzarəti**: Thread-təhlükəsiz vəziyyət idarəetməsi üçün atom əməliyyatları
- **İrəliləyiş İzləmə**: Real vaxt WebSocket kimi rabitə

## 🌐 Azərbaycan Dili Dəstəyi

Nomino düzgün sıralama ilə Azərbaycan əlifbası üçün hərtərəfli dəstək daxildir:

**Azərbaycan Əlifbası Sırası:**
A, B, C, Ç, D, E, Ə, F, G, Ğ, H, X, I, İ, J, K, Q, L, M, N, O, Ö, P, R, S, Ş, T, U, Ü, V, Y, Z

**Xüsusiyyətlər:**
- Təbii sıralama Azərbaycan simvol sırasına hörmət edir
- Böyük-kiçik hərf həssas olmayan müqayisələr
- Azərbaycan mətni ilə rəqəm sıralaması
- Tam Unicode dəstəyi

## 🔧 Konfiqurasiya

### Excel Fayl Tələbləri
- Format: `.xlsx` (Excel 2007+)
- Struktur: Başlıqları olan sütunlarda məlumat
- Kodlaşdırma: UTF-8 uyğun
- Ölçü: Xüsusi limitlər yoxdur (yaddaş asılı)

### Fayl Sistemi İcazələri
- Mənbə qovluqlara oxuma girişi
- Təyinat qovluqlara yazma girişi
- Müvəqqəti fayl yaradılması icazələri

### Sistem Tələbləri
- **Windows**: 10 və ya sonrası (x64)
- **macOS**: 10.15 və ya sonrası
- **Linux**: Ubuntu 18.04+ və ya ekvivalent
- **RAM**: Minimum 4GB, 8GB tövsiyə edilir
- **Yaddaş**: Tətbiq üçün 50MB, əməliyyatlar üçün əlavə

## 🚨 Problemlərin Həlli

### Ümumi Problemlər

**1. Excel Faylı Oxunmur**
- Faylın .xlsx formatında olduğundan əmin olun
- Fayl icazələrini yoxlayın
- Sütun hərflərini doğrulayın (A, B, C, və s.)
- Fayl açıqsa Excel-i bağlayın

**2. Proses İlişib/Asılıb**
- Əməliyyatları dayandırmaq üçün Dayandır düyməsini istifadə edin
- Təyinat qovluq icazələrini yoxlayın
- Kifayət qədər disk sahəsini doğrulayın

**3. Simvol Kodlaşdırma Problemləri**
- Excel faylının UTF-8 uyğun olduğundan əmin olun
- Azərbaycan simvol göstərilməsini yoxlayın
- Şrift dəstəyini doğrulayın

**4. Performans Problemləri**
- Digər tətbiqləri bağlayın
- Kiçik topluları emal edin
- Mövcud RAM-ı yoxlayın

### Xəta Mesajları
- **"Qovluq mövcud deyil"** - Qovluq mövcud deyil
- **"Excel faylını açmaq mümkün olmadı"** - Excel faylı açıla bilmir
- **"Proses işləmir"** - Proses işləmir
- **"Yanlış sütun hərfi"** - Yanlış sütun hərfi

## 🤝 Töhfə Vermə

Töhfələri salamlayırıq! Təfərrüatlar üçün [Töhfə Vermə Təlimatlarımıza](CONTRIBUTING.md) baxın.

### İnkişaf Təlimatları
1. Rust və TypeScript ən yaxşı təcrübələrini izləyin
2. Yeni xüsusiyyətlər üçün testlər əlavə edin
3. Sənədləri yeniləyin
4. Şərti commit mesajlarından istifadə edin
5. Çarpaz platforma uyğunluğunu təmin edin

### Problemləri Bildirmə
Lütfən aşağıdakılarla [problem izləyicimizə](https://github.com/ClauseBreaker/nomino_desktop/issues) müraciət edin:
- Təfərrüatlı təsvir
- Təkrarlama addımları
- Sistem məlumatları
- Tətbiq olunarsa ekran görüntüləri

## 📄 Lisenziya

Bu layihə müəlliflik tələbləri ilə sərbəst istifadə, dəyişdirmə və yayıma icazə verən Xüsusi Lisenziya altındadır - təfərrüatlar üçün [LICENSE](LICENSE) faylına baxın.

**Əsas Tələblər:**
- Bu proqramı sərbəst istifadə, dəyişdirmə və yaya bilərsiniz
- Footerdə görünən şəkildə yazılmalıdır: "ClauseBreaker tərəfindən yaradılıb"
- Footerdə GitHub və veb sayt linklərini daxil etməlisiniz
- Yaradıcıdan heç bir zəmanət və ya məsuliyyət yoxdur
- Azərbaycan və beynəlxalq müəlliflik hüququ qanunları ilə idarə olunur

## 👨‍💻 Müəllif

**ClauseBreaker**
- GitHub: [@ClauseBreaker](https://github.com/ClauseBreaker)
- Veb sayt: [clausebreaker.github.io](https://clausebreaker.github.io)

## 🙏 Təşəkkürlər

- [Tauri](https://tauri.app/) - Əla masaüstü tətbiq framework-u üçün
- [SvelteKit](https://kit.svelte.dev/) - Müasir frontend framework-u üçün
- [Calamine](https://github.com/tafia/calamine) - Excel fayl emalı üçün
- [Lucide](https://lucide.dev/) - Gözəl ikonlar üçün

## 📊 Statistika

- **Dillər**: Rust (70%), TypeScript (25%), CSS (5%)
- **Ümumi Sətirlər**: ~15,000+
- **Modullar**: 8 funksional modul
- **Əmrlər**: 25+ backend əmr
- **Dəstəklənən Fayl Növləri**: Excel (.xlsx), PDF, Şəkillər (JPG, PNG, və s.)

---

<div align="center">

**Azərbaycan icması üçün ❤️ ilə hazırlanıb**

Faydalı hesab edirsinizsə [⭐ Bu repositoriyanı ulduzlayın](https://github.com/ClauseBreaker/nomino_desktop)!

</div> 