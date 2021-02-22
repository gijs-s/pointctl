#!/usr/bin/env -S pipenv run python3
# -*- coding: utf-8 -*-

import os
import tempfile
import zipfile
import requests
import hashlib
from pathlib import Path
from typing import Callable, List

from dataclasses import dataclass
import numpy as np
import pandas as pd
from tqdm import tqdm
from scipy.io import arff
from sklearn.preprocessing import LabelEncoder, MinMaxScaler


def process_abalone(input_file: Path, output_file: Path):
    df = pd.read_csv(input_file, header=None)
    df.columns = [
        "sex",
        "length",
        "diameter",
        "height",
        "whole_weight",
        "shucked_weigth",
        "viscera_weight",
        "shell_weight",
        "y",
    ]

    X = df.drop("y", axis=1)
    X = pd.get_dummies(X)
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = df["y"]

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_absenteeism(input_file: Path, output_file: Path):
    f = zipfile.ZipFile(input_file)
    tmp_dir = tempfile.TemporaryDirectory()
    f.extractall(tmp_dir.name)

    df = pd.read_csv(
        os.path.join(tmp_dir.name, "Absenteeism_at_work.csv"), sep=";", index_col=None
    )
    y = np.array(df["Absenteeism time in hours"] > 0).astype("uint8")
    X = pd.get_dummies(df.drop(["ID", "Absenteeism time in hours"], axis=1))
    column_names = [
        c.replace(" ", "_").replace("/", "_").lower() for c in list(X.columns)
    ]

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_bank(input_file: Path, output_file: Path):
    bank = zipfile.ZipFile(input_file)
    tmp_dir = tempfile.TemporaryDirectory()
    bank.extractall(tmp_dir.name)

    df = pd.read_csv(
        os.path.join(tmp_dir.name, "bank-additional", "bank-additional-full.csv"),
        sep=";",
    )
    y = np.array(df["y"] == "yes").astype("uint8")
    X = pd.get_dummies(df.drop("y", axis=1))
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_banknote(input_file: Path, output_file: Path):
    df = pd.read_csv(input_file, index_col=None)
    df.columns = ["variance", "skewness", "curtosis", "entropy", "class"]

    y = np.array(df["class"] == 1).astype("uint8")
    X = pd.get_dummies(df.drop(["class"], axis=1))
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_defaultcc(input_file: Path, output_file: Path):
    df = pd.read_excel(input_file, header=1)

    y = np.array(df["default payment next month"] == 1).astype("uint8")
    X = pd.get_dummies(df.drop(["ID", "default payment next month"], axis=1))
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_diabetes(input_file: Path, output_file: Path):
    df = pd.read_csv(input_file, index_col=None)
    y = np.array(df["class"] == "Positive").astype("uint8")
    X = pd.get_dummies(df.drop(["class"], axis=1))
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_epileptic(input_file: Path, output_file: Path):
    df = pd.read_csv(input_file, index_col=None)
    X = df.drop(["Unnamed: 0"], axis=1)
    X = X.drop("y", axis=1)
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = df["y"]

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_happiness(input_file: Path, output_file: Path):
    df = pd.read_csv(
        input_file,
        encoding="utf-16",
        index_col=None,
    )

    y = np.array(df["D"] == 1).astype("uint8")
    X = pd.get_dummies(df.drop(["D"], axis=1))
    column_names = [
        "city_services",
        "housing_cost",
        "school_quality",
        "police_trust",
        "street_maint",
        "community_events",
    ]

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_seismic(input_file: Path, output_file: Path):
    data, _ = arff.loadarff(input_file)
    df = pd.DataFrame.from_records(data)

    df["seismic"] = df["seismic"].str.decode("utf-8")
    df["seismoacoustic"] = df["seismoacoustic"].str.decode("utf-8")
    df["shift"] = df["shift"].str.decode("utf-8")
    df["ghazard"] = df["ghazard"].str.decode("utf-8")
    df["class"] = df["class"].str.decode("utf-8")

    y = np.array((df["class"] == "1").astype("uint8"))
    X = pd.get_dummies(df.drop("class", axis=1))
    column_names = list(X.columns)

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


def process_wbc(input_file: Path, output_file: Path):
    tmp = pd.read_csv(input_file, header=None, na_values="?")

    X = np.array(tmp[[1, 2, 3, 4, 5, 6, 7, 8, 9]])
    y = np.array(tmp[[10]]).squeeze()

    lenc = LabelEncoder()
    y = lenc.fit_transform(y)

    X[np.isnan(X)] = 0.0

    scaler = MinMaxScaler()
    X = scaler.fit_transform(X)

    column_names = [
        "clump_thickness",
        "uniformity_cell_size",
        "uniformity_cell_shape",
        "marginal_adhesion",
        "single_epithelial_cell_size",
        "bare_nuclei",
        "bland_chromatin",
        "normal_nucleoli",
        "mitoses",
    ]

    X = pd.DataFrame(data=X, columns=column_names)
    X["y"] = y

    print("Processed dataset to {0} rows, {1} columns".format(len(X), len(X.columns)))
    X.to_csv(output_file, sep=";", index=None)


@dataclass
class Dataset:
    # The name of the dataset
    name: str
    # The url where the dataset can be downloaded
    url: str
    # A hash of the dataset to ensure the file is not changed while working on it
    hash: str
    # Path to the raw unprocessed file
    raw_file_path: Path
    # Path to the cleaned up csv file with a header
    clean_file_path: Path
    # Processing function used
    processing_function: Callable[[Path, Path], None]

    def verify_downloaded(self) -> None:
        """
        Download the url to raw_file_path if it is not already present
        """
        if not dataset.raw_file_path.exists():
            print(f"Source file for {dataset.name} is missing, downloading it")
            self.download_file()

    def download_file(self) -> None:
        """
        Preform a streaming download to download the file available at the url.
        It will store this file to the file specified in path.
        """
        # Create the path if it did not exist.
        self.raw_file_path.parent.mkdir(exist_ok=True, parents=True)

        # Start a streaming get request, retrieve the file size from t he content-length headers
        r = requests.get(self.url, stream=True)
        file_size = int(r.headers.get("Content-Length"))

        # Create a fancy progress bar.
        pbar = tqdm(
            total=file_size,
            unit="B",
            unit_scale=True,
            desc=self.url.split("/")[-1],
        )
        with open(self.raw_file_path, "wb") as f:
            for chunk in r.iter_content(chunk_size=1024):
                if chunk:
                    f.write(chunk)
                    pbar.update(1024)

        pbar.close()
        # Verify that the hash of the file is correct
        self.check_hash()

    def check_hash(self) -> None:
        """
        Check if hash of the file downloaded to raw_file_path
        matches the hash specified in the dataclass
        """
        with open(self.raw_file_path, "rb") as f:
            file_hash = hashlib.sha256(f.read()).hexdigest()
            if file_hash != self.hash:
                print(
                    f"The file hosted as {dataset.url} has an unexpected hash.\n"
                    "Please manually check if the processing is still correct"
                )
                exit(1)

    def process(self) -> None:
        """
        Run the processing function for this class
        """
        self.verify_downloaded()
        self.processing_function(self.raw_file_path, self.clean_file_path)


if __name__ == "__main__":
    base_dir = Path("../data")

    datasets: List[Dataset] = [
        Dataset(
            name="abalone",
            url="https://archive.ics.uci.edu/ml/machine-learning-databases/abalone/abalone.data",
            hash="de37cdcdcaaa50c309d514f248f7c2302a5f1f88c168905eba23fe2fbc78449f",
            raw_file_path=(base_dir / "abalone" / "raw" / "abalone.data").resolve(),
            clean_file_path=(base_dir / "abalone" / "abalone.csv").resolve(),
            processing_function=process_abalone,
        ),
        Dataset(
            name="absenteeism",
            url="https://archive.ics.uci.edu/ml/machine-learning-databases/00445/Absenteeism_at_work_AAA.zip",
            hash="89ecdfed5f107bb97015c335b1d812d7ecbe86e601a23b56516967d8657e53c4",
            raw_file_path=(base_dir / "absenteeism" / "raw" / "Absenteeism_at_work_AAA.zip").resolve(),
            clean_file_path=(base_dir / "absenteeism" / "absenteeism.csv").resolve(),
            processing_function=process_absenteeism,
        ),
        Dataset(
            name="bank",
            url="http://archive.ics.uci.edu/ml/machine-learning-databases/00222/bank-additional.zip",
            hash="a607b5edab6c6c75ce09c39142a77702c38123bd5aa7ae89a63503bbe17d65cd",
            raw_file_path=(base_dir / "bank" / "raw" / "bank-additional.zip").resolve(),
            clean_file_path=(base_dir / "bank" / "bank.csv").resolve(),
            processing_function=process_bank,
        ),
        Dataset(
            name="banknote",
            url="https://archive.ics.uci.edu/ml/machine-learning-databases/00267/data_banknote_authentication.txt",
            hash="d0539aaed2139ba7a587b3e34fb345ce503ff7d5d33dbf9912d8e195ce425cb9",
            raw_file_path=(
                base_dir / "banknote" / "raw" / "data_banknote_authentication.txt"
            ).resolve(),
            clean_file_path=(base_dir / "banknote" / "banknote.csv").resolve(),
            processing_function=process_banknote,
        ),
        # Dataset(
        #     name="defaultcc",
        #     url="https://archive.ics.uci.edu/ml/machine-learning-databases/00350/default of credit card clients.xls",
        #     hash="30c6be3abd8dcfd3e6096c828bad8c2f011238620f5369220bd60cfc82700933",
        #     raw_file_path=(
        #         base_dir / "raw" / "default of credit card clients.xls"
        #     ).resolve(),
        #     clean_file_path=(base_dir / "defaultcc" / "defaultcc.csv").resolve(),
        #     processing_function=process_defaultcc,
        # ),
        Dataset(
            name="diabetes",
            url="https://archive.ics.uci.edu/ml/machine-learning-databases/00529/diabetes_data_upload.csv",
            hash="7889d9d0beb7dd1ccc58da99f72763f16afb259b5dbbaa086f8195366ff66137",
            raw_file_path=(base_dir / "diabetes" / "raw" / "diabetes_data_upload.csv").resolve(),
            clean_file_path=(base_dir / "diabetes" / "diabetes.csv").resolve(),
            processing_function=process_diabetes,
        ),
        Dataset(
            name="epileptic",
            url="http://archive.ics.uci.edu/ml/machine-learning-databases/00388/data.csv",
            hash="4b3f6024ea24a864c0de51b2ba477bf8b9e4974e8cc01a00dcf45bfc59d48deb",
            raw_file_path=(base_dir / "epileptic" /"raw" / "data-epileptic.csv").resolve(),
            clean_file_path=(base_dir / "epileptic" / "epileptic.csv").resolve(),
            processing_function=process_epileptic,
        ),
        Dataset(
            name="happiness",
            url="https://archive.ics.uci.edu/ml/machine-learning-databases/00479/SomervilleHappinessSurvey2015.csv",
            hash="1feffca7dd0b9455b4bb16a72c3c5b33ddd78d8a13ea2c7578894481acb5c538",
            raw_file_path=(
                base_dir / "happiness" /"raw" / "SomervilleHappinessSurvey2015.csv"
            ).resolve(),
            clean_file_path=(base_dir / "happiness" / "happiness.csv").resolve(),
            processing_function=process_happiness,
        ),
        Dataset(
            name="seismic",
            url="http://archive.ics.uci.edu/ml/machine-learning-databases/00266/seismic-bumps.arff",
            hash="aabe512fab65b36d1dfb462650b75cfd8d99d8cc2723e8ecb4e6f5e1caccd5a7",
            raw_file_path=(base_dir /"seismic"/ "raw" / "seismic-bumps.arff").resolve(),
            clean_file_path=(base_dir / "seismic" / "seismic.csv").resolve(),
            processing_function=process_seismic,
        ),
        # Dataset(
        #     name="wbc",
        #     url="https://archive.ics.uci.edu/ml/machine-learning-databases/breast-cancer-wisconsin/breast-cancer-wisconsin.data",
        #     hash="402c585309c399237740f635ef9919dc512cca12cbeb20de5e563a4593f22b64",
        #     raw_file_path=(base_dir / "wbc" / "raw" / "breast-cancer-wisconsin.data").resolve(),
        #     clean_file_path=(base_dir / "wbc" /"wbc.csv").resolve(),
        #     processing_function=process_wbc,
        # ),
    ]

    # If the base directory does not exist yet create it.
    if not base_dir.exists():
        base_dir.mkdir()

    print("Retrieving all datasets to ../data")
    for dataset in datasets:
        print(f"\n\033[1;36mGathering {dataset.name}\033[0m")
        dataset.process()

    print("\nLinks for all datasets:")
    print(
        """
https://archive.ics.uci.edu/ml/datasets/Abalone
https://archive.ics.uci.edu/ml/datasets/Absenteeism+at+work
https://archive.ics.uci.edu/ml/datasets/Bank+Marketing
https://archive.ics.uci.edu/ml/datasets/banknote+authentication
https://archive.ics.uci.edu/ml/datasets/default+of+credit+card+clients
https://archive.ics.uci.edu/ml/datasets/Early+stage+diabetes+risk+prediction+dataset.
https://archive.ics.uci.edu/ml/datasets/Epileptic+Seizure+Recognition
https://archive.ics.uci.edu/ml/datasets/Somerville+Happiness+Survey
https://archive.ics.uci.edu/ml/datasets/seismic-bumps
https://archive.ics.uci.edu/ml/datasets/Breast+Cancer+Wisconsin+%28Diagnostic%29
        """
    )
