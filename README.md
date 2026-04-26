# cemircol

Yüksek performanslı, sütun tabanlı (columnar) veri depolama kütüphanesi. Rust/PyO3 çekirdeği, `zstd` maksimum sıkıştırması ve sıfır-kopya `mmap + numpy` pipeline'ı sayesinde **Parquet'ten daha hızlı okuma** ve **15x daha küçük dosya boyutu** sunar.

## Kurulum

```bash
pip install cemircol
```

CSV ve Parquet dönüştürücülerini kullanmak için:
```bash
pip install "cemircol[froms]"
```

*(Kaynak koddan derlemek için Rust toolchain gereklidir: `maturin develop --release`)*

## Özellikler

- **zstd Level 22 Sıkıştırma:** Her sütunu bağımsız olarak maksimum düzeyde sıkıştırır — Parquet'ten 15x daha küçük dosya boyutu.
- **Sıfır-Kopya Okuma Pipeline:** `mmap → PyByteArray (doğrudan decompress) → numpy.frombuffer` zinciri ile hiçbir ara Rust buffer oluşturulmaz.
- **Paralel Yazma:** `rayon` ile tüm sütunlar aynı anda sıkıştırılır, tüm CPU core'ları kullanılır.
- **numpy Entegrasyonu:** `query()` doğrudan numpy array döner — Python list nesnesi yaratma yükü yoktur.
- **Mmap (Memory Mapped Files):** Dosyaları belleğe eşleyerek RAM tüketimini minimize eder.
- **Geriye Dönük Uyumluluk:** Eski zlib formatındaki `.cemir` dosyalar otomatik tanınır ve okunur.

## Kullanım

### Hızlı Başlangıç

```python
from cemircol import CemircolWriter, CemircolReader

# Veri yazma
data = {"id": [1, 2, 3], "val": [1.1, 2.2, 3.3]}
CemircolWriter.write("data.cemir", data)

# Veri okuma — numpy array döner
reader = CemircolReader("data.cemir")
print(reader.columns())    # ['id', 'val']
print(reader.num_rows())   # 3
print(reader.query("val")) # array([1.1, 2.2, 3.3])
```

### CSV ve Parquet'ten Çevirme

```python
from cemircol import from_csv, from_parquet

from_csv("sales_data.csv", "sales_data.cemir")
from_parquet("analytics.parquet", "analytics.cemir")
```

## Performans Karşılaştırması

10.000.000 satır, 2 sütun (`int64` + `float64`) benchmark (`benchmark.py`):

### Dosya Boyutları

```
  CemirCol  :   5.52 MB   ← 15x Parquet'ten küçük
  Parquet   :  85.09 MB
  CSV       : 182.61 MB
  JSON      : 192.15 MB
```

### Tek Sütun Okuma Süresi

```
  CemirCol  : 0.108 s   ← Parquet'ten hızlı
  Parquet   : 0.113 s
  JSON      : 1.219 s
  CSV       : 6.597 s
```

> `query()` numpy array döndürdüğünden veri direkt bilimsel hesaplama için hazırdır.

## Teknik Mimari

### Dosya Formatı (`.cemir`)

```
┌──────────┬──────────────┬──────────────┬───────────────────────┬──────────────┬──────────┐
│ "CEM1"   │ col_1_data   │ col_2_data   │ metadata_json         │ meta_len u64 │ "CEM1"   │
│ 4 bytes  │ (zstd)       │ (zstd)       │ (FileMeta: offsets,   │ 8 bytes      │ 4 bytes  │
│          │              │              │  types, compression)  │              │          │
└──────────┴──────────────┴──────────────┴───────────────────────┴──────────────┴──────────┘
```

### Okuma Pipeline

```
mmap (sıfır kopya)
  └─► sıkıştırılmış dilim (compressed slice)
        └─► PyByteArray::new_with → zstd::stream::copy_decode (doğrudan Python belleğine)
              └─► numpy.frombuffer (sıfır-kopya view)
                    └─► numpy array → Python
```

## Proje Yayınlama

Sürüm yayınlamadan önce `Cargo.toml` ve `pyproject.toml` içindeki versiyon numaralarını senkronize güncelle, ardından:

```bash
./publish.sh
# Twine kullanıcı adı : __token__
# Twine şifre        : pypi-... (PyPI API token)
```

Geliştirme ortamı için:
```bash
maturin develop --release
```

---

- Muslu YÜKSEKTEPE
- Cem Emir YÜKSEKTEPE
