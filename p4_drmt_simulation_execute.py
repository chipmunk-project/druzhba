# Example: python3 p4_drmt_siulation_execute.py p4_files/stateful.py 20 3_
import argparse
import sys
import subprocess

def run_druzhba (args):
    subprocess.run(['cargo',
                    'run',
                    'drmt_p4',
                    args[0], # P4 file
                    args[1], # Number of packet fields
                    args[2], # Number of ticks
                    args[3], # Number of processors
                    args[4], # Number of state vars
                    args[5]]) # Path to dRMT repository

def main ():
    argv = sys.argv
    parser = argparse.ArgumentParser(description='dsim execution')
    parser.add_argument(
            'p4_file',
            type=str,
            help='P4 file')
    parser.add_argument(
            'num_packet_fields',
            type=int,
            help='Number of packet fields')

    parser.add_argument(
            'ticks',
            type=int,
            help='Number of ticks')
    parser.add_argument(
            'num_processors',
            type=int,
            help='Number of dRMT processors')
    parser.add_argument(
            'num_state_vars',
            type=int,
            help='Number of state variables for dRMT architecture')
    parser.add_argument(
            'path_to_drmt',
            type=str,
            help='Path to dRMT repository')

    # TODO: Add argument for number of cycles the program will run for
    raw_args = parser.parse_args(argv[1:])
    args = []
    args.append(raw_args.p4_file)
    args.append(str(raw_args.num_packet_fields))
    args.append(str(raw_args.ticks))
    args.append(str(raw_args.num_packet_fields))
    args.append(str(raw_args.num_state_vars))
    args.append(str(raw_args.path_to_drmt))

    print('Args: ' , args)
    run_druzhba (args)

if __name__ == '__main__':
    main()
  
