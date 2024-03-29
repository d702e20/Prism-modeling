import re
import sys


def make_header(num_of_mexicans, num_of_lives, prismfile):
    prismfile.write(f"// Mexican Standoff with {num_of_mexicans} mexicans each having {num_of_lives} lives\n\n")
    prismfile.write("csg \n\n")


def make_players(num_of_mexicans, prismfile):
    for x in range(num_of_mexicans):
        player = f"player p{x + 1}\n \tshooter{x + 1}\nendplayer\n\n"
        prismfile.write(player)


def make_const_globals(num_of_mexicans, num_of_lives, prismfile):
    prismfile.write(f"const int max_health = {num_of_lives};\n\n")

    for x in range(num_of_mexicans):
        prismfile.write(f"global health{x + 1} : [0..max_health] init max_health;\n")


def make_labels(num_of_mexicans, prismfile):
    # "All_died" label
    alldied = "label \"all_died\" = "
    for x in range(num_of_mexicans):
        if x + 1 < num_of_mexicans:
            alldied += f"health{x + 1} = 0 & "
        elif x + 1 == num_of_mexicans:
            alldied += f"health{x + 1} = 0;"
    prismfile.write("\n" + alldied + "\n")

    # "ShooterX_won" labels
    for x in range(num_of_mexicans):
        label = f"label \"shooter{x + 1}_won\" ="
        for y in range(num_of_mexicans):
            if x == y:
                label += f" health{x + 1} > 0"
            else:
                label += f" health{y + 1} = 0"
            if y + 1 < num_of_mexicans:
                label += " &"
            else:
                label += ";\n"
        prismfile.write(label)

    # "aliveX" label
    for x in range(num_of_mexicans):
        prismfile.write(f"label \"alive{x + 1}\" = health{x + 1} > 0;\n")

    prismfile.write("\n")


def make_shooters(num_of_mexicans, prismfile):
    # The first shooter which defines actions
    first_shooter = "module shooter1\n\t[p1wait] true -> true;\n"
    for x in range(num_of_mexicans - 1):
        first_shooter += f"\t[p1shoot{x + 2}] health1 > 0 & health{x + 2} > 0 -> true;\n"
    first_shooter += "endmodule\n\n"
    prismfile.write(first_shooter)

    # All other shooters using relabeling
    for shooterID in range(2, num_of_mexicans + 1):
        shooter = f"module shooter{shooterID} = shooter1 ["

        # Relabel health
        health_relabeling = f"health1 = health{shooterID}, "
        for y in range(2, num_of_mexicans + 1):
            if y == shooterID:
                health_relabeling += f"health{y} = health1, "
        shooter += health_relabeling

        # Relabel actions
        action_relabeling = f"p1wait = p{shooterID}wait, "
        for y in range(2, num_of_mexicans + 1):
            if y == shooterID:
                action_relabeling += f"p1shoot{shooterID} = p{shooterID}shoot1"
            else:
                action_relabeling += f"p1shoot{y} = p{shooterID}shoot{y}"
            if y < num_of_mexicans:
                action_relabeling += ", "

        shooter += action_relabeling

        shooter += "] endmodule"
        prismfile.write(shooter + "\n")


def make_channel(num_of_mexicans, prismfile):
    prismfile.write("\nmodule channel\n")

    # Multidimensional array to hold possible actions per player
    all_player_actions = []
    combinations = pow(num_of_mexicans, num_of_mexicans)

    # Setup ratios for each shooter
    ratio = int(combinations / num_of_mexicans)
    ratio_array = []
    for x in range(num_of_mexicans):
        ratio_array.append(ratio)
        ratio = int(ratio / num_of_mexicans)

    # Fill up multidimensional array of actions available to shooters
    # Player doing action
    for x in range(1, num_of_mexicans + 1):
        player_actions = []
        # Player being shot
        for y in range(1, num_of_mexicans + 1):
            if x == y:
                player_actions.append(f"p{x}wait")
            else:
                player_actions.append(f"p{x}shoot{y}")
        all_player_actions.append(player_actions)

    # Holds index for action to be used by shooter
    next_action_to_be_used = [0 for row in range(num_of_mexicans)]
    # Total number of rows of actions
    for x in range(combinations):
        current_action = "  ["

        # How many players in each action (line)
        for y in range(num_of_mexicans):
            current_action += all_player_actions[y][next_action_to_be_used[y]]

            # If there are more shooters, add comma
            if y + 1 < num_of_mexicans:
                current_action += ","

            # Cycle through actions based on ratio
            if ratio_array[y] <= (x % ratio_array[y]) + 1:
                next_action_to_be_used[y] = (next_action_to_be_used[y] + 1) % num_of_mexicans

        current_action += "] true -> "

        # Do update of healths
        health_update = ""

        # Find targets
        targets = re.findall("(?<=shoot)\d", current_action, re.MULTILINE)

        if len(targets) == 0:
            current_action += "true"
        hits_so_far = 0
        for x in range(1, num_of_mexicans + 1):
            x_gets_hit = 0
            for target in targets:
                if int(target) == x:
                    x_gets_hit += 1
            if 0 < x_gets_hit:
                health_update += f"(health{x}' = (health{x}-{x_gets_hit}))"

                hits_so_far += x_gets_hit

                if hits_so_far < len(targets):
                    health_update += " & "

        current_action += health_update + ";\n"
        prismfile.write(current_action)

    prismfile.write("endmodule")


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python mexican_standoff.py <mexicans> <lives>")
        exit(1)
    N = int(sys.argv[1])
    if N <= 1:
        print("Number of mexicans must be greater than 1")
        exit(1)
    H = int(sys.argv[2])
    if H <= 0:
        print("Number of lives must be greater than 0")
        exit(1)

    file_name = f'mexican_standoff_{N}p_{H}hp.prism'
    with open(file_name, 'w') as f:
        make_header(N, H, f)
        make_players(N, f)
        make_const_globals(N, H, f)
        make_labels(N, f)
        make_shooters(N, f)
        make_channel(N, f)

    print(f"{file_name} was generated successfully!")
