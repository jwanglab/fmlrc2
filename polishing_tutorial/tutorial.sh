# FMLRC2 bacterial assembly tutorial
# ----------------------------------

# **Install dependencies**

# If you don't have Rust and its package manager - cargo - installed, follow steps here: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

#    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install FMLRC2 from GitHub using cargo:

    git clone https://github.com/jwanglab/fmlrc2
    cd fmlrc2
    cargo test --release
    cargo build --release
    cd ..

# Install ropebwt2 (requires zlib):

    git clone https://github.com/lh3/ropebwt2
    cd ropebwt2
    make
    cd ..

# Install Flye (also requires zlib):

    git clone https://github.com/fenderglass/Flye
    cd Flye
    make
    cd ..

# **Download test data**

    mkdir -p data
    cd data
    wget https://sc.unc.edu/jrwang/fmlrc2-data/-/raw/main/Citrobacter_koseri_MINF_9D.long.fastq.gz
    wget https://sc.unc.edu/jrwang/fmlrc2-data/-/raw/main/Citrobacter_koseri_MINF_9D.R1.fastq.gz
    wget https://sc.unc.edu/jrwang/fmlrc2-data/-/raw/main/Citrobacter_koseri_MINF_9D.R2.fastq.gz
    cd ..

# **Nanopore assembly**

    ./Flye/bin/flye --nano-raw data/Citrobacter_koseri_MINF_9D.long.fastq.gz --genome-size 5000000 --threads 8 --out-dir C_koseri_Flye

# **Polishing with FMLRC2**

    mkdir -p C_koseri_FMLRC2
    gunzip -c data/Citrobacter_koseri_MINF_9D.R*.fastq.gz | awk "NR % 4 == 2" | tr NT TN | ./ropebwt2/ropebwt2 -LR | tr NT TN | ./fmlrc2/target/release/fmlrc2-convert C_koseri_FMLRC2/comp_msbwt.npy
    ./fmlrc2/target/release/fmlrc2 -t 8 C_koseri_FMLRC2/comp_msbwt.npy C_koseri_Flye/assembly.fasta C_koseri_FMLRC2/polished.fasta

# Running this example on an AMD EPYC 7313 (2.4Ghz) with 8 threads, Flye takes ~12 minutes and BWT construction and FMLRC2 together take less than a minute.
