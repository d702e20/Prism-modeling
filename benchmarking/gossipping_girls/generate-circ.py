
def generate_players(f, N: int):
    for i in range(1, N+1):
        f.write(f"""
player p{i}
    girl{i}
endplayer
""")


def generate_secrets(f, N: int):
    for g in range(1, N+1):
        f.write(f'\n// Girl{g}\'s secrets\n')
        for s in range(1, N+1):
            f.write(f'global g{g}s{s} : bool init {str(g == s).lower()};\n')


def generate_shorthands(f, N: int):
    f.write('\n// Handy shorthands\n')
    for g in range(1, N+1):
        for s in range(1, N+1):
            f.write(f'formula g{g}g{(g%N)+1}s{s} = g{g}s{s} | g{(g%N)+1}s{s};\n')
        if g != N:
            f.write('\n')


def generate_labels(f, N: int):
    f.write('\n// Labels\n')
    for g in range(1, N+1):
        f.write(f'label "omniscient{g}" = ')
        for s in range(1, N+1):
            f.write(f'g{g}s{s}')
            if s != N:
                f.write(' & ')
        f.write(';\n')
    f.write('label "allomniscient" = ')
    for g in range(1, N+1):
        for s in range(1, N+1):
            f.write(f'g{g}s{s}')
            if g != N or s != N:
                f.write(' & ')
    f.write(';\n')


def generate_round_counter(f, N: int):
    f.write("""\n// Round counting
const int roundmax = 20;
global round : [0..roundmax] init 0;

module counter
	[] round<=roundmax -> (round'=min(round+1,roundmax));
endmodule
""")


def generate_modules(f, N: int):
    for g in range(1, N+1):
        if g == 1:
            f.write(f"""
// Modules
module girl1
    [_1call_left] round <= roundmax -> true;
    [_1call_right] round <= roundmax -> true;
endmodule

""")
        else:
            f.write(f'module girl{g} = girl1 [_1call_left =_{g}call_left,_1call_right = _{g}call_right] endmodule\n')


def generate_synchronizations(f, N: int):
    f.write('\n// Synchronizations\nmodule channel\n')
    f.write('\t// TODO\n')
    f.write('endmodule\n')


if __name__ == '__main__':
    N = int(input('Number of girls: '))
    if N <= 1:
        print('Aborting generation')
    print(f'Generating circular network of {N} girls')

    with open(f'generated-circ-{N}.prism', 'w') as f:
        f.write("""// Gossipping Girls circular

csg
""")
        generate_players(f, N)
        generate_secrets(f, N)
        generate_shorthands(f, N)
        generate_labels(f, N)
        generate_round_counter(f, N)
        generate_modules(f, N)
        generate_synchronizations(f, N)
