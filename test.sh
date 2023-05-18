#!/bin/bash
#SBATCH --job-name=rust_stack_alg_sim
#SBATCH --output=job_output_%j.txt
#SBATCH --ntasks=6
#SBATCH --cpus-per-task=1
#SBATCH --time=01:00:00

srun ./target/release/myrustprogram Stack 5 2000 Sawtooth 5 &
srun ./target/release/myrustprogram Stack 5 2000 Sawtooth 5 &
srun ./target/release/myrustprogram Stack 5 2000 Sawtooth 5 &
srun ./target/release/myrustprogram Stack 5 2000 Sawtooth 5 &
srun ./target/release/myrustprogram Stack 5 2000 Sawtooth 5 &
srun ./target/release/myrustprogram Stack 5 2000 Sawtooth 5

wait
