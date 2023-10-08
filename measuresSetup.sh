echo "Setting up dependencies..."

echo "#### Installing Git"
sudo apt install git

echo "#### Installing CMake"
sudo apt install cmake

echo "#### Installing lm-sensors"
sudo apt install lm-sensors

echo "### Installing Sloc Cloc and Code (scc)"
sudo snap install scc

echo "#### Installing Powercap"
git clone https://github.com/powercap/powercap.git
cd powercap
mkdir _build
cd _build
cmake ..
make
make install

cd ..
cd ..

echo "#### Installing package config"
sudo apt install pkg-config

echo "#### Installing Raplcap"
git clone https://github.com/powercap/raplcap.git
cd raplcap 
mkdir _build
cd _build
cmake ..
make
make install

cd ..
cd ..
