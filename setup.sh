cd
mkdir "~/.pinst"
if [ -f ".bashrc" ]; then
  echo 'export PATH="~/.pinst/:{$PATH}"' >> "~/.bashrc"
elif [ -f ".bash_profile" ]; then
  echo 'export PATH="~/.pinst/:{$PATH}"' >> "~/.bash_profile"
else
  echo "~/.pinst couldn't be added to the path, please add it manualy"
fi
