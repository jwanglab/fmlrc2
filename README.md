# FMLRC2

FMLRC2 performs error correction/polishing of long erroneous sequences with accurate short reads. As such, it can be used as *both* an error-correction tool \[[1](https://bmcbioinformatics.biomedcentral.com/articles/10.1186/s12859-018-2051-3)\] for raw long reads (ex. Oxford Nanopore) *and* a polishing tool \[[2](http://biorxiv.org/content/early/2022/07/23/2022.07.22.501182)\] for *de novo* assemblies.

This repo contains the source code for FMLRC v2, based on the same methodology used by the original [FMLRC](https://github.com/holtjma/fmlrc).  In benchmarks, the results between FMLRC v1 and v2 are nearly identical, but tests have shown that v2 uses approximately 50% of the run and CPU time compared to v1.

Try the [Polishing Tutorial](https://github.com/jwanglab/fmlrc2/tree/master/polishing_tutorial) for full step-by-step instructions for assembling and polishing a bacterial genome!


## Installation
All installation options assume you have installed [Rust](https://www.rust-lang.org) along with the `cargo` crate manager for Rust.

### From GitHub
```bash 
git clone https://github.com/jwanglab/fmlrc2.git
cd fmlrc2
#testing optional, some tests will fail if ropebwt2 is not installed or cannot be found on PATH
cargo test --release
cargo build --release
./target/release/fmlrc2 -h
./target/release/fmlrc2-convert -h
```

## Usage
### BWT Building
#### msbwt2 Construction Approach
For most users, it is recommended to use the [msbwt2](https://github.com/HudsonAlpha/rust-msbwt) crate to [build the BWT](https://github.com/HudsonAlpha/rust-msbwt#msbwt-building).
This approach is generally simpler (requiring only one command) and more flexible (accepting both FASTQ and FASTA files at once).
While it is generally competitive with the `ropebwt2` construction approach (see below) for memory and CPU usage, it is not parallelized and typically runs slower by wall-clock time.

#### ropebwt2 Construction Approach
If you are familiar with more complicated shell commands, then `ropebwt2` can also be used to build the BWT.  
For _most_ short-read datasets, this approach is faster than `msbwt2-build` but also more complicated (multiple piped commands) and less flexible (fixed to FASTQ in the below example).
Given one or more FASTQ files of accurate reads (`reads.fq.gz` with extras labeled as `[reads2.fq.gz ...]`), you can use the following command from this crate to create a BWT at `comp_msbwt.npy`.
Note that this command requires the [ropebwt2](https://github.com/lh3/ropebwt2) executable to be installed:
```
gunzip -c reads.fq.gz [reads2.fq.gz ...] | \
    awk 'NR % 4 == 2' | \
    tr NT TN | \
    ropebwt2 -LR | \
    tr NT TN | \
    fmlrc2-convert comp_msbwt.npy
```

### Read Error Correction
Assuming the accurate-read BWT is built (`comp_msbwt.npy`) and uncorrected reads are available (fasta/fastq, gzip optional, `uncorrected.fq.gz`), FMLRC2 can be run as follows:
```
fmlrc2 [OPTIONS] <comp_msbwt.npy> <uncorrected.fq.gz> <corrected_reads.fa>
```

### Genome Assembly Polishing
Assuming the accurate-read BWT is built (`comp_msbwt.npy`) and unpolished assembly are available (fasta/fastq, gzip optional, `assembly.fa`), FMLRC2 can be run as follows:
```
fmlrc2 [OPTIONS] <comp_msbwt.npy> <assembly.fa> <polished_assembly.fa>
```

For bacterial and other short, minimally repetitive genomes, the default parameters typically produce optimal results. For polishing more complex eukaryotic genomes, adding options `--k 21 59 80, --min_frac 0` usually improves polishing accuracy, particularly in repetitive elements, with a modest increase in run time (~10%).

#### Options to consider
1. `-h` - see full list of options and exit
2. `-k`, `--K` - sets the k-mer sizes to use, default is `[21, 59]`; all values are sorted from lowest to highest prior to correction
3. `-t`, `--threads` - number of correction threads to use (default: 1)
4. `-C`, `--cache_size` - the length of sequences to pre-compute (i.e. `C`-mers); will reduce CPU-time of queries by `O(C)` but *increases* cache memory usage by `O(6^C)`; default of `8` uses ~25MB; if memory is not an issue, consider using `10` with ~1GB cache footprint (or larger if memory _really_ isn't an issue)

## FMLRC v2 core differences
1. Implemented in [Rust](https://www.rust-lang.org) instead of C++ - this comes will all the benefits of Rust including `cargo`, such as easy installation of the binary and supporting structs/functions along with documentation
2. Unlimited `k`/`K` parameters - FMLRC v1 allowed 1 or 2 sizes for `k` only; FMLRC v2 can have the option set as many times as desired at increased CPU time (for example, a 3-pass correction with `k=[21, 59, 79]`) 
3. Call caching - FMLRC v2 pre-computes all _k_-mers of a given size. This reduces the run-time significantly by cutting reducing calls to the FM-index.
4. Input handling - thanks to [needletail](https://crates.io/crates/needletail), the uncorrected reads can be in FASTA/FASTQ and may or may not be gzip compressed.
5. Unit testing - FMLRC v2 has unit testing through the standard Rust testing framework (i.e. `cargo test`)

## FAQ
### How do I set multiple, custom k-mer sizes?
K-mer sizes are set via the `-k`/`-K` parameter.
Since the parameter allows for multiple values, the CLI may need the end-of-list delimiter ("`--`") specified as well.
The following is an example of how you would add a third filtering step with `k=79` to the defaults:
```
fmlrc2 -k 21 59 79 -- <comp_msbwt.npy> <uncorrected.fq.gz> <corrected_reads.fa>
```

## Publications
A detailed description of the algorithm, performance compared to other error correction and genome polishing tools, and run time/resource usage benchmarks are available in the relevant publications:

[Qing Charles Mak, Ryan R Wick, James Matthew Holt and Jeremy R Wang. Polishing *de novo* nanopore assemblies of bacteria and eukaryotes with FMLRC2. biorxiv, 2022.](http://biorxiv.org/content/early/2022/07/23/2022.07.22.501182)

[Wang, Jeremy R. and Holt, James and McMillan, Leonard and Jones, Corbin D. FMLRC: Hybrid long read error correction using an FM-index. BMC Bioinformatics, 2018. 19 (1) 50.](https://bmcbioinformatics.biomedcentral.com/articles/10.1186/s12859-018-2051-3)

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
