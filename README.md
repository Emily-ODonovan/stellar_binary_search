# stellar_binary_search
A rust preprocessing app for the YALE bright star catalogue

## How to run!

If you would like to compile the code make sure you have visual studio development tools (2017 or later), rustup, a c++ compiler installed and propertly configured in the path!

You will also need your key.json to connect to your firebase instance, if you are assessing this project the provided one should work! If not see StellarStarSearch setup instructions.

The cargo is already configured so you can just compile and run it.

If you would like to skip the compilation step you will need access to "stellar_binary_search/target/debug/stellar_binary_search.exe".
If you do place it in the same directory as "asuNoHeader.tsv". If you do not already have it go to https://vizier.cds.unistra.fr. Search for catalogue V/50. Select the ";" seperated values paramater, set "max" to "unlimited" and download by selecting "V/50/catalog" and press "query selected tables.

Once you have "asu.tsv" open it in a text editor, delete the first 74 lines and rename it to "asuNoHeader.tsv".

## Finally!

Place "asuNoHeader.tsv", "key.json" and "stellar_binary_search.exe" into the same directory and once you have your firebase emulator running, execute stellar_binary_search.exe and wait until you get the "success!" message in your command line.
