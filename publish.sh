#!/bin/bash
set -e

echo "=== CemirCol PyPI Yükleme Aracı ==="

# 1. Maturin ve Twine kontrolü
if ! command -v maturin &> /dev/null || ! command -v twine &> /dev/null; then
    echo "[HATA] 'maturin' veya 'twine' komutu bulunamadı."
    echo "Lütfen sanal ortamı (venv) aktifleştirdiğinizden ve bağımlılıkları yüklediğinizden emin olun: pip install -r requirements.txt"
    echo "Linux/Mac için: source .venv/bin/activate"
    exit 1
fi

echo "[*] Derleme işlemi başlıyor..."
maturin build --release

echo "[*] PyPI'ye yükleme işlemi başlıyor..."
echo "NOT: PyPI artık şifre aslıyla çalışmıyor."
echo "Yükleme sırasında Username istenince: __token__ yazınız."
echo "Password istenince: pypi- ile başlayan API tokeninizi giriniz."
echo "Eğer test ağına yüklemek isterseniz, bu scripti iptal edip 'twine upload --repository testpypi target/wheels/*' kullanabilirsiniz."

# Twine ile yükle
twine upload target/wheels/*

echo "=== Yükleme Tamamlandı! ==="
