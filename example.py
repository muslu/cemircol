import os
import pandas as pd
from cemircol import CemircolWriter, CemircolReader, from_csv, from_parquet

def main():
    print("=== 1. Doğrudan Dictionary Kullanarak Veri Yazma ===")
    data = {
        "user_id": [1001, 1002, 1003, 1004],
        "transaction_amount": [150.5, 20.0, 345.99, 12.5],
        "is_active": [True, False, True, True]
    }
    
    # Write compressed columnar data
    CemircolWriter.write("transactions.cemir", data)
    print("Veri 'transactions.cemir' dosyasına başarıyla yazıldı.\n")

    print("=== 2. CemirCol Dosyasından Veri Okuma ===")
    # Read specific columns using memory-mapped zero-copy logic
    reader = CemircolReader("transactions.cemir")
    print(f"Mevcut Sütunlar : {reader.columns()}")
    print(f"Toplam Satır    : {reader.num_rows()}")
    
    amounts = reader.query("transaction_amount")
    print(f"'transaction_amount' sütun verileri => {amounts}\n")

    print("=== 3. Format Çeviriciler (Converter) ===")
    # Test amacıyla geçici CSV ve Parquet oluşturalım
    df = pd.DataFrame(data)
    df.to_csv("sample.csv", index=False)
    df.to_parquet("sample.parquet", index=False)
    print("Pandas kullanılarak 'sample.csv' ve 'sample.parquet' oluşturuldu.\n")

    # CSV -> CemirCol dönüşümü
    print("--- CSV'den CemirCol'a ---")
    from_csv("sample.csv", "from_csv.cemir")
    csv_reader = CemircolReader("from_csv.cemir")
    print("Çevrilen dosyadan alınan user_id verisi: ", csv_reader.query("user_id"))

    # Parquet -> CemirCol dönüşümü
    print("\n--- Parquet'ten CemirCol'a ---")
    from_parquet("sample.parquet", "from_parquet.cemir")
    pq_reader = CemircolReader("from_parquet.cemir")
    print("Çevrilen dosyadan alınan user_id verisi: ", pq_reader.query("user_id"))

    # Temizlik (İsteğe bağlı test kalıntılarını silebilirsiniz)
    files_to_clean = ["sample.csv", "sample.parquet", "transactions.cemir", "from_csv.cemir", "from_parquet.cemir"]
    for f in files_to_clean:
        if os.path.exists(f):
            os.remove(f)
            
    print("\n=== Örnek Uygulama Başarıyla Tamamlandı ===")

if __name__ == "__main__":
    main()