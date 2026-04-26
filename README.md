# cemircol

Yüksek performanslı, sütun tabanlı (columnar) veri depolama kütüphanesi. Rust ile yazılan C-ABI (PyO3) tabanlı çekirdeği sayesinde zero-copy mmap işlemleri kullanarak muazzam okuma performansı sunar. Ayrıca `flate2` (Zlib) üzerinden blok sıkıştırması yaparak verilerinizi **Parquet'ten bile daha az alanda** depolar.

## Kurulum

Sadece okuma ve yazma özelliklerini kullanmak için (hafif kurulum):
```bash
pip install cemircol
```

CSV ve Parquet dönüştürücülerini kullanmak için:
```bash
pip install "cemircol[froms]"
```

*(Kaynak koddan derlemek için Rust toolchain gereklidir: `maturin develop --release`)*

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

Bu özellikleri kullanmak için paketi `cemircol[froms]` seçeneği ile kurmuş olmanız gerekir:

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

## Proje Güncelleme ve PyPI Üzerinde Yayınlama

Projeyi güncelleyeceğiniz zaman veya yeni bir sürüm yayınlamak istediğinizde aşağıdaki adımları sırasıyla izlemelisiniz:

### 1. Güncelleme Adımları ve Sebepleri

- **Adım 1: `Cargo.toml` Dosyasını Güncellemek**
  - **Ne Yapılır:** `version = "x.y.z"` satırındaki sürüm numarası artırılır (Örn: `0.1.1` -> `0.1.2`).
  - **Sebep:** Projenin çekirdeği (hız ve bellek yönetimi sağlayan kısımlar) Rust ile yazılmıştır. Rust derleyicisi olan Cargo'nun yeni derleme yaparken doğru sürüm bilgisini üretmesi ve Rust paket bağımlılıklarının doğru çalışması için bu gereklidir.
- **Adım 2: `pyproject.toml` Dosyasını Güncellemek**
  - **Ne Yapılır:** `[project]` altındaki `version = "x.y.z"` satırındaki sürüm numarası artırılır.
  - **Sebep:** Projenin son kullanıcı tarafı Python'dur ve PyPI (Python Package Index) üzerinde yayınlanırken paket bilgileri büyük oranda bu dosyadan okunur. `Cargo.toml` ve `pyproject.toml` sürümlerinin senkronize (aynı) olması paket yönetiminde karışıklıkları önler.
- **Adım 3: Eski Derleme Dosyalarını Temizlemek**
  - **Ne Yapılır:** `target/wheels/` klasörünün içi boşaltılır (Projedeki `./publish.sh` betiği bunu otomatik yapar).
  - **Sebep:** Twine aracı o klasördeki her şeyi yüklemeye çalışır. Eğer eski derlemeler (örneğin eski `0.1.0` dosyası) orada kalırsa, PyPI "File already exists" hatası vererek yüklemeyi durduracaktır.
- **Adım 4: Derleme ve Yükleme (Maturin & Twine)**
  - **Ne Yapılır:** `maturin build --release` ile proje derlenir ve `twine upload target/wheels/*` ile PyPI sunucularına aktarılır.

### 2. Neden Maturin Kullanıldı?

Projenin temel gücü Rust dilinde yazılmış olmasından gelir. Ancak son kullanıcının (veri bilimci vs.) bu gücü bilindik ve basit bir Python arayüzünden (`import cemircol`) kullanabilmesi gerekir. 

- **Maturin**, Rust dilinde yazılmış kodları (PyO3 kütüphanesi yardımıyla) otomatik olarak Python'un anlayabileceği "Extension Module" (C-Eklentisi) formatına çevirir.
- Geleneksel yöntemlerle Python için bir C/C++ uzantısı yazmak ve derlemek oldukça zor, hataya açık ve platform bağımlı bir süreçtir.
- Maturin, hiçbir karmaşık `setup.py` dosyasına ihtiyaç duymadan, Rust tabanlı bir projeyi saniyeler içinde direkt olarak bir Python Wheel (`.whl`) dosyasına (dağıtılabilir paket formatına) dönüştürür.

### 3. Maturin Olmadan Olmaz mı?

Evet, teorik olarak **olabilir ancak oldukça meşakkatli olur**. 

Eğer Maturin kullanmak istemezsek:
1. Geleneksel `setuptools-rust` kütüphanesini kurmamız gerekir.
2. Karmaşık bir `setup.py` dosyası yazarak, Python'un `build` sistemi ile Rust'ın `cargo` derleyicisini manuel olarak haberleştirmemiz ve yönetmemiz gerekir.
3. Linux, Windows ve macOS için teker teker doğru tekerlek (wheel) etiketlemelerini (`manylinux`, `musllinux` vb.) C düzeyindeki derleme aşamalarında manuel olarak yapılandırmamız gerekir.

Maturin tüm bu süreçleri bir standart haline getirdiği ve sıfır konfigürasyon ile modern `pyproject.toml` standartını desteklediği için modern Rust-Python projelerinde de-facto (standart) araçtır.

### Yayınlama Komutları

Maturin için eski `publish` komutu kullanımdan (deprecated) kaldırılmıştır ve PyPI standart şifre yerine API Token metoduna geçmiştir. Yayınlamak için depoda bulunan `./publish.sh` betiğini çalıştırabilirsiniz. Bu betik eski derlemeleri temizler, yenisini derler ve `twine` ile gönderir.

Terminalde `twine` sizden istendiğinde:
- **Kullanıcı adı**: `__token__` girin.
- **Şifre**: PyPI hesap ayarları > API tokens kısmından oluşturduğunuz `pypi-` ile başlayan tam yetkili token verisini girin.

- Muslu YÜKSEKTEPE
- Cem Emir YÜKSEKTEPE