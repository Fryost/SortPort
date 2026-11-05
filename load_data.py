from pathlib import Path
import pandas as pd

dataset = "hf://datasets/jason23322/high-accuracy-email-classifier/"

train = pd.read_csv(dataset + "train.csv") # todo: decide on train ratio
test = pd.read_csv(dataset + "test.csv") # todo: decide on test ratio

print(train.columns)
print(train.head(5))
print(train["category"].value_counts())
print(len(train), "train /", len(test), "test")

row = train.iloc[0]
print("SUBJECT:", row["subject"])
print("BODY:", row["body"][:300])
print("TEXT:", row["text"][:300])

# <impurities>
print(train.isna().sum())
print("Duplicates:", train["text"].duplicated().sum())
print("Overlaps in test set:", test["text"].isin(train["text"]).sum())
# </impurities>

# <duplicates and overlapping removed>
train = train.drop_duplicates(subset="text")
test = test.drop_duplicates(subset="text")
test = test[~test["text"].isin(train["text"])]
# </duplicates and overlapping removed>

print(len(train), "train /", len(test), "test after cleaning")

# <export for viewer>
output = Path(__file__).parent / "output"
output.mkdir(exist_ok=True)
train.to_csv(output / "train.csv", index=False)
test.to_csv(output / "test.csv", index=False)
# </export for viewer>