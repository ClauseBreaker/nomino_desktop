/**
 * Icon Generation Script
 * 
 * This script generates all required application icons from a source logo image.
 * It creates multiple formats and sizes needed for Tauri desktop applications.
 * 
 * Usage: node scripts/generate-icons.js
 * 
 * Requirements:
 * - Source logo: assets/logo.png
 * - Sharp library (auto-installed)
 * 
 * Generated files:
 * - src-tauri/icons/32x32.png
 * - src-tauri/icons/128x128.png  
 * - src-tauri/icons/icon.png
 * - src-tauri/icons/icon.ico (Windows)
 * - src-tauri/icons/icon.svg (Vector)
 * - static/favicon.png
 */

const fs = require('fs');
const path = require('path');

// Configuration
const CONFIG = {
    SOURCE_LOGO: 'assets/logo.png',
    TAURI_ICONS_DIR: 'src-tauri/icons',
    STATIC_DIR: 'static',
    SIZES: {
        SMALL: 32,
        MEDIUM: 128,
        LARGE: 256,
        ICO_SIZES: [16, 32, 48, 64, 128]
    }
};

/**
 * Ensures Sharp library is available, installs if needed
 * @returns {Promise<Object>} Sharp instance
 */
async function ensureSharp() {
    try {
        return require('sharp');
    } catch (error) {
        console.log('📦 Installing Sharp for image processing...');
        const { execSync } = require('child_process');
        execSync('npm install sharp', { stdio: 'inherit' });
        return require('sharp');
    }
}

/**
 * Creates directory if it doesn't exist
 * @param {string} dirPath - Directory path to create
 */
function ensureDirectory(dirPath) {
    if (!fs.existsSync(dirPath)) {
        fs.mkdirSync(dirPath, { recursive: true });
    }
}

/**
 * Generates PNG icons in various sizes
 * @param {Object} sharp - Sharp instance
 * @param {string} logoPath - Source logo path
 */
async function generatePngIcons(sharp, logoPath) {
    console.log('🖼️  Generating PNG icons...');
    
    const iconsDir = CONFIG.TAURI_ICONS_DIR;
    const staticDir = CONFIG.STATIC_DIR;
    
    ensureDirectory(iconsDir);
    ensureDirectory(staticDir);
    
    const resizeOptions = { 
        fit: 'contain', 
        background: { r: 0, g: 0, b: 0, alpha: 0 } 
    };
    
    // Generate different sizes
    const iconTasks = [
        {
            size: CONFIG.SIZES.SMALL,
            output: path.join(iconsDir, '32x32.png')
        },
        {
            size: CONFIG.SIZES.MEDIUM,
            output: path.join(iconsDir, '128x128.png')
        },
        {
            size: CONFIG.SIZES.MEDIUM,
            output: path.join(iconsDir, 'icon.png')
        },
        {
            size: CONFIG.SIZES.SMALL,
            output: path.join(staticDir, 'favicon.png')
        }
    ];
    
    // Execute all resize operations
    await Promise.all(
        iconTasks.map(({ size, output }) =>
            sharp(logoPath)
                .resize(size, size, resizeOptions)
                .png()
                .toFile(output)
        )
    );
}

/**
 * Creates ICO file structure from multiple PNG images
 * @param {Array} images - Array of image objects with size and buffer
 * @returns {Buffer} ICO file buffer
 */
function createIcoFile(images) {
    const numImages = images.length;
    
    // ICO header (6 bytes)
    const header = Buffer.alloc(6);
    header.writeUInt16LE(0, 0);          // Reserved (must be 0)
    header.writeUInt16LE(1, 2);          // Type (1 = ICO)
    header.writeUInt16LE(numImages, 4);  // Number of images
    
    // Directory entries (16 bytes each)
    const directorySize = numImages * 16;
    const directory = Buffer.alloc(directorySize);
    
    let imageDataOffset = 6 + directorySize;
    let directoryOffset = 0;
    const imageDataBuffers = [];
    
    // Build directory entries and collect image data
    for (const image of images) {
        const imageSize = image.buffer.length;
        
        // Directory entry
        directory.writeUInt8(image.size === 256 ? 0 : image.size, directoryOffset);     // Width
        directory.writeUInt8(image.size === 256 ? 0 : image.size, directoryOffset + 1); // Height
        directory.writeUInt8(0, directoryOffset + 2);   // Color count (0 for true color)
        directory.writeUInt8(0, directoryOffset + 3);   // Reserved
        directory.writeUInt16LE(1, directoryOffset + 4); // Color planes
        directory.writeUInt16LE(32, directoryOffset + 6); // Bits per pixel
        directory.writeUInt32LE(imageSize, directoryOffset + 8);      // Image size
        directory.writeUInt32LE(imageDataOffset, directoryOffset + 12); // Image offset
        
        imageDataBuffers.push(image.buffer);
        imageDataOffset += imageSize;
        directoryOffset += 16;
    }
    
    return Buffer.concat([header, directory, ...imageDataBuffers]);
}

/**
 * Generates Windows ICO file with multiple sizes
 * @param {Object} sharp - Sharp instance
 * @param {string} logoPath - Source logo path
 */
async function generateIcoFile(sharp, logoPath) {
    console.log('🔄 Generating Windows ICO file...');
    
    const icoImages = [];
    const resizeOptions = { 
        fit: 'contain', 
        background: { r: 0, g: 0, b: 0, alpha: 0 } 
    };
    
    // Generate all sizes for ICO
    for (const size of CONFIG.SIZES.ICO_SIZES) {
        const imageBuffer = await sharp(logoPath)
            .resize(size, size, resizeOptions)
            .png()
            .toBuffer();
        
        icoImages.push({ size, buffer: imageBuffer });
    }
    
    // Create and save ICO file
    const icoData = createIcoFile(icoImages);
    const icoPath = path.join(CONFIG.TAURI_ICONS_DIR, 'icon.ico');
    fs.writeFileSync(icoPath, icoData);
}

/**
 * Generates SVG icon with embedded PNG
 * @param {Object} sharp - Sharp instance
 * @param {string} logoPath - Source logo path
 */
async function generateSvgIcon(sharp, logoPath) {
    console.log('🎯 Generating SVG icon...');
    
    // Convert PNG to base64 for embedding
    const pngBuffer = fs.readFileSync(logoPath);
    const base64 = pngBuffer.toString('base64');
    
    // Create SVG with embedded PNG
    const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg width="128" height="128" viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg">
  <title>Application Icon</title>
  <image href="data:image/png;base64,${base64}" width="128" height="128" />
</svg>`;
    
    const svgPath = path.join(CONFIG.TAURI_ICONS_DIR, 'icon.svg');
    fs.writeFileSync(svgPath, svg);
}

/**
 * Main function to generate all icons
 */
async function generateAllIcons() {
    try {
        const sharp = await ensureSharp();
        const logoPath = CONFIG.SOURCE_LOGO;
        
        // Validate source logo exists
        if (!fs.existsSync(logoPath)) {
            console.error(`❌ Source logo not found: ${logoPath}`);
            console.log('Please ensure the logo file exists at the specified path.');
            process.exit(1);
        }
        
        console.log(`🎨 Using source logo: ${logoPath}`);
        
        // Generate all icon formats
        await generatePngIcons(sharp, logoPath);
        await generateIcoFile(sharp, logoPath);
        await generateSvgIcon(sharp, logoPath);
        
        // Success message
        console.log('\n✅ All icons generated successfully!');
        console.log('\nGenerated files:');
        console.log('  📁 Tauri Icons:');
        console.log('    - src-tauri/icons/32x32.png');
        console.log('    - src-tauri/icons/128x128.png');
        console.log('    - src-tauri/icons/icon.png');
        console.log('    - src-tauri/icons/icon.ico (Windows, multi-size)');
        console.log('    - src-tauri/icons/icon.svg (Vector)');
        console.log('  📁 Web Assets:');
        console.log('    - static/favicon.png');
        
    } catch (error) {
        console.error('❌ Error generating icons:', error.message);
        process.exit(1);
    }
}

// Execute if run directly
if (require.main === module) {
    generateAllIcons();
}

module.exports = { generateAllIcons }; 