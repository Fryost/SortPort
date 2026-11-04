import pandas as pd

dataset = "hf://datasets/jason23322/high-accuracy-email-classifier/"

train = pd.read_csv(dataset + "train.csv") # todo: decide on train ratio
test = pd.read_csv(dataset + "test.csv") # todo: decide on test ratio

print(train.columns)
print(train.head(5))
print(train["category"].value_counts())
print(len(train), "train /", len(test), "test")