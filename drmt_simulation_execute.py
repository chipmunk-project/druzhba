import argparse
import sys
import subprocess

def run_druzhba (args):
    subprocess.run(['cargo',
                    'run',
                    'drmt',
                    args[0], # RISCV file
                    args[1], # Number of packets
                    args[2], # Number of ticks
                    args[3]]) # Number of processors

def main ():
    argv = sys.argv
    parser = argparse.ArgumentParser(description='dsim execution')
    parser.add_argument(
            'riscv_file',
            type=str,
            help='RISCV asm file to be run on each processor')

    parser.add_argument(
            'num_packet_fields',
            type=int,
            help='Number of PHV containers (should be equal to the number of packet fields)')
    parser.add_argument(
            'ticks',
            type=int,
            help='Number of ticks')
    parser.add_argument(
            'num_processors',
            type=int,
            help='Number of processors for dRMT architecture')
    # TODO: Add argument for number of cycles the program will run for
    raw_args = parser.parse_args(argv[1:])
    args = []
    args.append(raw_args.riscv_file)
    args.append(str(raw_args.num_packet_fields))
    args.append(str(raw_args.ticks))
    args.append(str(raw_args.num_processors))

    subprocess.run(['./build_dgen.sh'])
    spike_result = subprocess.run(['which',
                                   'spike'],
                                   stdout=subprocess.DEVNULL)
    cross_compiler_result = subprocess.run(['which',
                                            'riscv64-unknown-elf-gcc'],
                                            stdout=subprocess.DEVNULL)
    if spike_result.returncode != 0:
        print('WARNING: Spike could not be found')
    if cross_compiler_result.returncode != 0:
        print('WARNING: The riscv64-unknown-elf-gcc cross compiler could not be found')

    run_druzhba (args)

if __name__ == '__main__':
    main()
  
