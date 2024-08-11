import sys

def process_log(log):
    lines = log.strip().splitlines()
    result = []
    task_counter = 3  # Commence par Task: 3

    i = 0
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith('R'):
            # Traitement des lignes commençant par 'R'
            r_line = f"R {lines[i][2:]} {lines[i+1]}"
            result.append(r_line)
            i += 2
        elif line.startswith('F') or line.startswith('P'):
            # Traitement des lignes commençant par 'F' ou 'P'
            result.append(f"{line} {lines[i+1]}")
            i += 2
        elif line.startswith('D'):
            # Traitement des lignes commençant par 'D' et ajout de la somme
            d_line = f"{line} {int(lines[i+1]) + int(lines[i+2])}"
            result.append(d_line)
            i += 3
        elif line.startswith('TASK'):
            # Traitement des lignes TASK
            result.append(line)
            i += 1
        else:
            i += 1
    
    return '\n'.join(result)

def main(file_path):
    with open(file_path, 'r') as file:
        log_content = file.read()
    
    processed_log = process_log(log_content)
    print(processed_log)

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python script.py <log_file_path>")
        sys.exit(1)

    log_file_path = sys.argv[1]
    main(log_file_path)
