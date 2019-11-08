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
            'num_packets',
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
    args.append(str(raw_args.num_packets))
    args.append(str(raw_args.ticks))
    args.append(str(raw_args.num_processors))

    print('drmt script args: ' , args)
    subprocess.run(['./build_dgen.sh'])
    run_druzhba (args)

if __name__ == '__main__':
    main()
  
