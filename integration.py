import subprocess

bashCommand = "git clone"
process = subprocess.Popen(bashCommand.split(), stdout=subprocess.PIPE)
output, error = process.communicate()

print(output)
