import os
import shutil
import argparse

def organize_files(source_folder, destination_folder):
    # List all files in the source folder
    files = [f for f in os.listdir(source_folder) if os.path.isfile(os.path.join(source_folder, f))]

    # Create the necessary folders in the destination folder
    for file in files:
        # Extract the values of o, f, l, p, and a from the file name
        parts = file.split('-')
        o = parts[0][1:]
        f = parts[1][1:]
        l = parts[2][1:]
        p = parts[3][1:]
        a = parts[4].split('.')[0][1:]

        # Create the folder structure
        o_path = os.path.join(destination_folder, f"o{o}")
        f_path = os.path.join(o_path, f"f{f}")
        l_path = os.path.join(f_path, f"l{l}")
        p_path = os.path.join(l_path, f"p{p}")

        # If the folders do not exist, create them
        os.makedirs(p_path, exist_ok=True)

        # Build the destination path of the file
        destination_file = os.path.join(p_path, f"a{a}.png")

        # Copy the file to the destination folder
        shutil.copy(os.path.join(source_folder, file), destination_file)

if __name__ == "__main__":
    # Configure the command line arguments parser
    parser = argparse.ArgumentParser(description="Organize files into a folder structure. Run the script with the following arguments: python organizer.py <source_folder/> <destination_folder/>")
    parser.add_argument("source_folder", help="Relative path of the source folder")
    parser.add_argument("destination_folder", help="Relative path of the destination folder")

    # Get the command line arguments
    args = parser.parse_args()

    # Convert relative paths to absolute paths
    source_path = os.path.abspath(args.source_folder)
    destination_path = os.path.abspath(args.destination_folder)

    # Call the function to organize the files with the provided arguments
    organize_files(source_path, destination_path)
