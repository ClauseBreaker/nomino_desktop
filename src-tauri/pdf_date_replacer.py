#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
PDF Date Replacer - Exact replica of the working Python program
Replaces the last date found on the last page of a PDF file
"""

import sys
import os
import re
import argparse
from pathlib import Path

try:
    import fitz  # PyMuPDF
except ImportError:
    print("ERROR: PyMuPDF not installed. Install with: pip install PyMuPDF")
    sys.exit(1)


def replace_date_in_pdf(pdf_path, old_date, new_date):
    """
    Replace the last occurrence of old_date with new_date on the last page of PDF
    This is the EXACT logic from the working Python program
    """
    try:
        # Open PDF document (like fitz.open(pdf_path))
        doc = fitz.open(pdf_path)
        
        # Get the last page (like doc[-1])
        page = doc[-1]
        
        # Extract text from last page (like page.get_text())
        text = page.get_text()
        
        # Find all dates using exact Python pattern
        date_pattern = r'(\d{2}[./]\d{2}[./]\d{4})'
        matches = list(re.finditer(date_pattern, text))
        
        if matches:
            # Get the last match (like matches[-1])
            last_match = matches[-1]
            found_old_date = last_match.group(0)
            
            # Check if the found date matches what we're looking for
            if found_old_date == old_date:
                # Search for the date position on the page (like page.search_for())
                rects = page.search_for(old_date)
                
                if rects:
                    # Take the last occurrence (like rects[-1])
                    rect = rects[-1]
                    
                    # Add redaction annotation to cover the old date (like page.add_redact_annot())
                    page.add_redact_annot(rect, fill=(1, 1, 1))  # White fill
                    
                    # Apply redactions (like page.apply_redactions())
                    page.apply_redactions()
                    
                    # Calculate new position below the original (use bottom of rect + offset)
                    new_position = fitz.Point(rect.tl.x, rect.br.y + 5)
                    
                    # Insert new date at the new position (like page.insert_text())
                    page.insert_text(new_position, new_date, fontsize=12, color=(0, 0, 0))
                    
                    # Save the new PDF
                    new_path = pdf_path[:-4] + '_new.pdf'
                    doc.save(new_path)
                    doc.close()
                    
                    print("SUCCESS: Date replaced {} -> {}".format(old_date, new_date))
                    try:
                        print("SAVED: {}".format(new_path))
                    except UnicodeEncodeError:
                        print("SAVED: [file with unicode characters]")
                    return True, new_path
                else:
                    doc.close()
                    print("ERROR: Date {} not found visually on last page".format(old_date))
                    return False, None
            else:
                doc.close()
                print("ERROR: Expected date {}, but found {}".format(old_date, found_old_date))
                return False, None
        else:
            doc.close()
            print("ERROR: No dates found on last page")
            return False, None
            
    except Exception as e:
        try:
            print("ERROR: {}".format(str(e)))
        except UnicodeEncodeError:
            print("ERROR: [unicode encoding error]")
        return False, None


def main():
    """Main function for command line usage"""
    parser = argparse.ArgumentParser(description='Replace date in PDF file')
    parser.add_argument('pdf_path', help='Path to PDF file')
    parser.add_argument('old_date', help='Old date to replace (DD.MM.YYYY or DD/MM/YYYY)')
    parser.add_argument('new_date', help='New date to insert (DD.MM.YYYY or DD/MM/YYYY)')
    parser.add_argument('--delete-original', action='store_true', help='Delete original file after processing')
    
    args = parser.parse_args()
    
    # Check if PDF file exists
    if not os.path.exists(args.pdf_path):
        print("ERROR: PDF file not found: {}".format(args.pdf_path))
        sys.exit(1)
    
    # Perform the replacement
    success, new_path = replace_date_in_pdf(args.pdf_path, args.old_date, args.new_date)
    
    if success:
        print("SUCCESS: PDF processed successfully")
        
        # Delete original file if requested
        if args.delete_original:
            try:
                os.remove(args.pdf_path)
                print("DELETED: Original file removed")
            except Exception as e:
                print("WARNING: Could not delete original file: {}".format(e))
        
        sys.exit(0)
    else:
        print("FAILED: Could not process PDF")
        sys.exit(1)


if __name__ == "__main__":
    main() 