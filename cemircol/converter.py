# cemircol/converter.py
from ._cemircol import CemircolWriter

def _check_dependencies():
    try:
        import pandas as pd
        return pd
    except ImportError:
        raise ImportError(
            "CemirCol dönüştürücü özelliklerini (from_csv, from_parquet) kullanabilmek için "
            "lütfen gerekli paketleri yükleyin: pip install 'cemircol[froms]'"
        )

def from_csv(csv_path: str, cemir_path: str):
    """
    Reads a CSV file using Pandas and converts it to a CemirCol file.
    """
    pd = _check_dependencies()
    df = pd.read_csv(csv_path)
    
    # Convert dataframe columns to standard Python lists
    data = {col: df[col].tolist() for col in df.columns}
    
    CemircolWriter.write(cemir_path, data)


def from_parquet(parquet_path: str, cemir_path: str):
    """
    Reads a Parquet file using Pandas/PyArrow and converts it to a CemirCol file.
    """
    pd = _check_dependencies()
    df = pd.read_parquet(parquet_path)
    
    data = {col: df[col].tolist() for col in df.columns}
    
    CemircolWriter.write(cemir_path, data)
