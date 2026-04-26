# cemircol

Yüksek performanslı, sütun tabanlı (columnar) veri depolama kütüphanesi. Rust ile yazılan C-ABI (PyO3) tabanlı çekirdeği sayesinde zero-copy mmap işlemleri kullanarak muazzam okuma performansı sunar. Ayrıca `flate2` (Zlib) üzerinden blok sıkıştırması yaparak verilerinizi **Parquet'ten bile daha az alanda** depolar.

## Kurulum

```bash
pip install cemircol
```

*(Kaynak koddan derlemek için Rust toolchain gereklidir: `bash src/setup.sh` veya `maturin develop --release`)*

## Özellikler

- **Mmap (Memory Mapped Files):** Dosyaları belleğe eşleyerek RAM tüketimini minimize eder.
- **Zlib Sıkıştırma:** Her sütunu bağımsız olarak maksimum seviyede (Level 9) sıkıştırır. Mükemmel "footprint" (ayak izi) sağlar.
- **Rust Core & PyO3 API:** Veri işleme mantığı tamamen güvenli ve performanslı Rust dilinde yazılmıştır, Python'da hiçbir bağımlılık gerektirmez.
- **Doğrudan Çeviri (Converter):** CSV ve Parquet dosyalarından anında CemirCol formatına dönüşüm.

## Kullanım & Örnekler

### Hızlı Başlangıç

```python
from cemircol import CemircolWriter, CemircolReader

# Veri yazma
data = {"id": [1, 2, 3], "val": [1.1, 2.2, 3.3]}
CemircolWriter.write("data.cemir", data)

# Veri okuma
reader = CemircolReader("data.cemir")
print(reader.columns())       # ['id', 'val']
print(reader.num_rows())      # 3
print(reader.query("val"))    # [1.1, 2.2, 3.3]
```

### CSV ve Parquet Formatından Çevirme (Converter)

Pandas kuruluysa ( `pip install pandas pyarrow` ) mevcut verilerinizi çok kolay bir şekilde CemirCol'a dönüştürebilirsiniz:

```python
from cemircol import from_csv, from_parquet

# CSV'den çevirme
from_csv("sales_data.csv", "sales_data.cemir")

# Parquet'ten çevirme
from_parquet("analytics.parquet", "analytics.cemir")
```

## Performans Karşılaştırma Testi (Benchmark)

Depoda bulunan `benchmark.py` dosyası ile 1 Milyon satırlık rastgele bir veri seti üzerinde yapılan test sonuçları:

### Dosya Boyutları (Disk Tüketimi)
```text
--- File Sizes ---
  CemirCol  : 2.87 MB   🥇 (En küçük dosya boyutu)
  Parquet   : 14.65 MB
  CSV       : 16.35 MB
  JSON      : 17.31 MB
```

### Okuma Süreleri (Tek Bir Sütun)
```text
--- Read Times (column: 'value') ---
  CemirCol  : 0.08362 s  🥇 (En hızlı okuma - Mmap)
  Parquet   : 0.28100 s
  JSON      : 0.56046 s
  CSV       : 3.00141 s
```
> Parquet ve CemirCol sütun tabanlı mimaridedir ancak Cemircol okuma anında zero-copy yaklaşımı kullandığından anlık tepki süresi (latency) çok daha iyidir.

## PyPI Üzerinde Yayınlama Nasıl Yapılır?

Maturin için `publish` komutu kullanımdan (deprecated) kaldırılmıştır ve PyPI standart şifre yerine API Token metoduna geçmiştir. Kütüphaneyi kendi adınıza bir PyPI paketine dönüştürüp yayınlamak isterseniz paketi `twine` ile yayınlayabilirsiniz:

1. Gereksinimleri yükleyin: `pip install -r requirements.txt` (twine ve maturin hazır olmalı).
2. Derleyin: `maturin build --release`
3. Twine ile PyPI'a gönderin: `twine upload target/wheels/*`

Terminalde sizden istendiğinde:
- **Kullanıcı adı**: `__token__` girin.
- **Şifre**: PyPI hesap ayarları > API tokens kısmından oluşturduğunuz `pypi-` ile başlayan tam yetkili token verisini girin.

Ayrıca bu işlemi tek seferde yapmak için depoda bulunan `./publish.sh` betiğini çalıştırabilirsiniz.

- Muslu YÜKSEKTEPE
- Cem Emir YÜKSEKTEPE