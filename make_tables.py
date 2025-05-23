#!/bin/python3
import pandas as pd
import matplotlib.pyplot as plt
import glob
import os

# Path to your raw CSV files
csv_files = [
"myvec_res.csv",
        "bv_res.csv",
        "vector_res.csv",
        "ef_res.csv",
        ]

labels = {
        "myvec_res.csv": "Custom Vector",
        "bv_res.csv": "BitVector",
        "vector_res.csv": "Vector",
        "ef_res.csv": "Elias-Fano",
        }

# Output directory for PDF plots
output_dir = "plots"
os.makedirs(output_dir, exist_ok=True)

# Read all CSVs
data_per_file = {}
for file in csv_files:
    df = pd.read_csv(file)
    df.columns = [col.strip() for col in df.columns]
    data_per_file[os.path.basename(file)] = df

# Gather unique log n and log u values
all_data = pd.concat(data_per_file.values(), ignore_index=True)
log_n_values = sorted(all_data['\\log n'].unique())
log_u_values = sorted(all_data['\\log u'].unique())

# Loop over each log n value
for log_n in log_n_values:
    # Initialize data tables
    space_table = pd.DataFrame(index=data_per_file.keys(), columns=log_u_values)
    access_table = pd.DataFrame(index=data_per_file.keys(), columns=log_u_values)
    successor_table = pd.DataFrame(index=data_per_file.keys(), columns=log_u_values)

    for filename, df in data_per_file.items():
        df_n = df[df['\\log n'] == log_n].set_index('\\log u')
        for log_u in log_u_values:
            if log_u in df_n.index:
                space_table.loc[filename, log_u] = df_n.loc[log_u, 'Space(MiB)']
                access_table.loc[filename, log_u] = df_n.loc[log_u, 'Access(ns)']
                successor_table.loc[filename, log_u] = df_n.loc[log_u, 'Successor(ns)']

    # Plotting function to PDF
    def save_plot(df, ylabel, title, filename_suffix):
        plt.figure(figsize=(10, 5))
        for file in df.index:
            y = df.loc[file].astype(float)
            plt.plot(log_u_values, y, marker='o', label=labels.get(file))
        plt.title(f'{title} (log n = {log_n})')
        plt.xlabel('log u')
        plt.ylabel(ylabel)
        plt.grid(True)
        plt.legend()
        plt.tight_layout()
        plt.savefig(f"{output_dir}/logn_{log_n}_{filename_suffix}.pdf")
        plt.close()

    # Save separate plots to PDF
    save_plot(space_table, 'Space (MiB)', 'Space Usage', 'space')
    save_plot(access_table, 'Access (ns)', 'Access Time', 'access')
    save_plot(successor_table, 'Successor (ns)', 'Successor Time', 'successor')

print(f"✅ PDF plots saved in '{output_dir}/' — one per metric and log n.")

import pandas as pd
import matplotlib.pyplot as plt
import os

# Input file
input_file = "ef_skewed_res.csv"

# Output directory
output_dir = "plots"
os.makedirs(output_dir, exist_ok=True)

# Read and clean the CSV
df = pd.read_csv(input_file)
df.columns = [col.strip() for col in df.columns]

# Get unique log n and log u values
log_n_values = sorted(df['\\log n'].unique())
log_u_values = sorted(df['\\log u'].unique())

# Plot a separate PDF for each log n
for log_n in log_n_values:
    df_n = df[df['\\log n'] == log_n].set_index('\\log u')
    y = df_n['Successor(ns)']

    plt.figure(figsize=(10, 5))
    plt.plot(log_u_values, y.loc[log_u_values], marker='o', label='Successor Time')
    plt.title(f'Successor Time vs log u (log n = {log_n})')
    plt.xlabel('log u')
    plt.ylabel('Successor (ns)')
    plt.grid(True)
    plt.legend()
    plt.tight_layout()
    plt.savefig(f"{output_dir}/skewed_{log_n}_successor.pdf")
    plt.close()

print(f"Skewed EF plots saved in '{output_dir}/' for each log n.")


