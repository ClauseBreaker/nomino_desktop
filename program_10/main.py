import sys
import os
import fitz  # PyMuPDF
from PyQt6.QtWidgets import (
    QApplication, QMainWindow, QPushButton, QFileDialog, QLabel, QProgressBar, QVBoxLayout, QWidget, QLineEdit, QMessageBox, QDateEdit, QCheckBox
)
from PyQt6.QtCore import Qt, QThread, pyqtSignal, QDate
from PyQt6.QtGui import QFont
from datetime import datetime

APP_AUTHOR = "Made by Tahir"

class PDFDateChangerThread(QThread):
    progress = pyqtSignal(int)
    status = pyqtSignal(str)
    finished = pyqtSignal()

    def __init__(self, root_folder, new_date, keyword, delete_original):
        super().__init__()
        self.root_folder = root_folder
        self.new_date = new_date
        self.keyword = keyword
        self.delete_original = delete_original

    def run(self):
        pdf_files = []
        for subdir, _, files in os.walk(self.root_folder):
            for file in files:
                if file.lower().endswith('.pdf') and self.keyword in file:
                    pdf_files.append(os.path.join(subdir, file))
        total = len(pdf_files)
        if total == 0:
            self.status.emit("Файлы не найдены!")
            self.progress.emit(0)
            self.finished.emit()
            return
        for idx, pdf_path in enumerate(pdf_files):
            self.status.emit(f"Обработка: {os.path.basename(pdf_path)}")
            self.process_pdf(pdf_path)
            self.progress.emit(int((idx + 1) / total * 100))
        self.status.emit("Готово!")
        self.finished.emit()

    def process_pdf(self, pdf_path):
        doc = fitz.open(pdf_path)
        page = doc[-1]
        text = page.get_text()
        import re
        date_pattern = r'(\d{2}[./]\d{2}[./]\d{4})'
        matches = list(re.finditer(date_pattern, text))
        if matches:
            last_match = matches[-1]
            old_date = last_match.group(0)
            # Заменяем только последнюю найденную дату через замазку и вписывание новой
            rects = page.search_for(old_date)
            if rects:
                rect = rects[-1]
                page.add_redact_annot(rect, fill=(1,1,1))
                page.apply_redactions()
                page.insert_text(rect.tl, self.new_date, fontsize=12, color=(0,0,0))
        new_path = pdf_path[:-4] + '_new.pdf'
        doc.save(new_path)
        doc.close()
        if self.delete_original:
            try:
                os.remove(pdf_path)
            except Exception as e:
                pass  # Можно добавить логирование ошибки

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle(f"PDF Date Changer")
        self.setFixedSize(500, 470)
        self.setStyleSheet("background-color: #f5f7fa;")
        font = QFont("Segoe UI", 11)
        self.setFont(font)

        layout = QVBoxLayout()
        self.label = QLabel("Select a folder with PDF files:")
        self.label.setStyleSheet("color: #000;")
        layout.addWidget(self.label)

        self.folder_btn = QPushButton("Choose Folder")
        self.folder_btn.setStyleSheet("background-color: #4f8cff; color: white; border-radius: 8px; padding: 10px; font-size: 15px;")
        self.folder_btn.clicked.connect(self.select_folder)
        layout.addWidget(self.folder_btn)

        self.keyword_label = QLabel("Keyword for file search:")
        self.keyword_label.setStyleSheet("color: #000;")
        layout.addWidget(self.keyword_label)

        self.keyword_input = QLineEdit()
        self.keyword_input.setText("İDDİA")
        self.keyword_input.setStyleSheet("padding: 8px; border-radius: 6px; border: 1px solid #ccc; background: #fff; color: #000;")
        layout.addWidget(self.keyword_input)

        self.date_label = QLabel("Select new date:")
        self.date_label.setStyleSheet("color: #000;")
        layout.addWidget(self.date_label)

        self.date_input = QDateEdit()
        self.date_input.setDisplayFormat("dd.MM.yyyy")
        self.date_input.setDate(QDate.currentDate())
        self.date_input.setCalendarPopup(True)
        self.date_input.setStyleSheet("padding: 8px; border-radius: 6px; border: 1px solid #ccc; background: #fff; color: #000;")
        layout.addWidget(self.date_input)

        self.delete_checkbox = QCheckBox("Delete original file after processing")
        self.delete_checkbox.setStyleSheet("color: #000; font-size: 13px; margin-top: 10px;")
        layout.addWidget(self.delete_checkbox)

        self.start_btn = QPushButton("Start Processing")
        self.start_btn.setStyleSheet("background-color: #43d19e; color: white; border-radius: 8px; padding: 10px; font-size: 15px;")
        self.start_btn.clicked.connect(self.start_processing)
        layout.addWidget(self.start_btn)

        self.progress = QProgressBar()
        self.progress.setValue(0)
        self.progress.setStyleSheet("QProgressBar {background: #e0e0e0; border-radius: 8px;} QProgressBar::chunk {background: #4f8cff; border-radius: 8px;}")
        layout.addWidget(self.progress)

        self.status = QLabel("Waiting...")
        self.status.setStyleSheet("color: #000; font-size: 13px;")
        layout.addWidget(self.status)

        self.author = QLabel(APP_AUTHOR)
        self.author.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.author.setFont(QFont("Segoe UI", 14, QFont.Weight.Bold))
        self.author.setStyleSheet("color: #4f8cff; font-size: 18px; margin-top: 30px;")
        layout.addWidget(self.author)

        container = QWidget()
        container.setLayout(layout)
        self.setCentralWidget(container)
        self.folder = None
        self.worker = None

    def select_folder(self):
        folder = QFileDialog.getExistingDirectory(self, "Выберите папку")
        if folder:
            self.folder = folder
            self.label.setText(f"Папка: {folder}")

    def start_processing(self):
        if not self.folder:
            QMessageBox.warning(self, "Ошибка", "Сначала выберите папку!")
            return
        new_date = self.date_input.date().toString("dd.MM.yyyy")
        keyword = self.keyword_input.text().strip()
        if not keyword:
            QMessageBox.warning(self, "Ошибка", "Введите ключевое слово!")
            return
        delete_original = self.delete_checkbox.isChecked()
        self.progress.setValue(0)
        self.status.setText("Загрузка...")
        self.worker = PDFDateChangerThread(self.folder, new_date, keyword, delete_original)
        self.worker.progress.connect(self.progress.setValue)
        self.worker.status.connect(self.status.setText)
        self.worker.finished.connect(self.on_finished)
        self.worker.start()

    def on_finished(self):
        QMessageBox.information(self, "Готово", "Обработка завершена!")
        self.status.setText("Готово!")

def main():
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
