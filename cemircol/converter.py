# cemircol/converter.py
from ._cemircol import CemircolWriter
import pandas as pd

def from_csv(csv_path: str, cemir_path: str):
    """
    Reads a CSV file using Pandas and converts it to a CemirCol file.
    """
    df = pd.read_csv(csv_path)
    
    # Convert dataframe columns to standard Python lists
    # Pandas naturally casts float and int types, making it very clean.
    data = {col: df[col].tolist() for col in df.columns}
    
    CemircolWriter.write(cemir_path, data)


def from_parquet(parquet_path: str, cemir_path: str):
    """
    Reads a Parquet file using Pandas/PyArrow and converts it to a CemirCol file.
    """
    df = pd.read_parquet(parquet_path)
    
    data = {col: df[col].tolist() for col in df.columns}
    
    CemircolWriter.write(cemir_path, data)
