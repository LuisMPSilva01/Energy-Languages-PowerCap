sudo apt-get update
pip install lazyme
sudo apt-get install make
sudo apt-get install zip unzip
sudo mkdir -p /opt/src
sudo apt-get update
sudo apt install cmake

## Compilers ##

    #Ada - v11.3.0
    sudo apt-get install libgmp-dev
    sudo apt-get install libapr1-dev
    wget https://gnat-packs.annexi-strayline.com/x86_64-linux-gnu/gnat-11.3.0-base.tar.xz
    mkdir gnat-11.3.0-base
    tar -xf gnat-11.3.0-base.tar -C gnat-11.3.0-base
    sudo mv gnat-11.3.0-base/opt/gcc-fsf-gnat /usr/local/
    export PATH=/usr/local/gcc-fsf-gnat/bin:$PATH
    rm -rf gnat-11.3.0-base gnat-11.3.0-base.tar 

    #C - v12.2.0 [NOTE: this will take about 3h to install]
    sudo apt install build-essential htop gcc-multilib zlib1g-dev 
    sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev 
    sudo apt-get install libapr1-dev
    sudo apt-get install libpcre2-dev
    wget http://mirror.linux-ia64.org/gnu/gcc/releases/gcc-12.2.0/gcc-12.2.0.tar.gz 
    tar -xvf gcc-12.2.0.tar.gz 
    cd gcc-12.2.0/ 
    ./configure --enable-shared --enable-linker-build-id --libexecdir=$HOME/gcc/usr/lib --without-included-gettext --enable-threads=posix --libdir=$HOME/gcc/usr/lib --enable-nls --disable-bootstrap --enable-clocale=gnu --enable-libstdcxx-debug --enable-libstdcxx-time=yes --with-default-libstdcxx-abi=new --enable-gnu-unique-object --disable-vtable-verify --enable-plugin --enable-default-pie --with-system-zlib --enable-libphobos-checking=release --with-target-system-zlib=auto --enable-objc-gc=auto --enable-multiarch --disable-werror --enable-cet --with-arch-32=i686 --with-abi=m64 --with-multilib-list=m32,m64,mx32 --enable-multilib --with-tune=generic --enable-checking=release --build=x86_64-linux-gnu --host=x86_64-linux-gnu --target=x86_64-linux-gnu --with-build-config=bootstrap-lto-lean --enable-link-serialization=2 --with-gmp --with-mpfr --with-mpc 
    make install 
    cd ../
    rm -rf gcc-12.2.0 gcc-12.2.0.tar.gz
        # K-nucl
        cd ./Languages/C/k-nucleotide || { echo "Directory not found"; exit 1; }
    
        # Clone the klib repository if not already cloned
        if [ ! -d "klib" ]; then
            git clone https://github.com/attractivechaos/klib.git
            echo "klib repository cloned."
        else
            echo "klib repository already exists."
        fi
        cd ../../../ 

    #C++ - v12.2.0
    sudo apt-get install libboost-all-dev
    sudo apt-get install libtbb-dev
    
    #C# (.net) - v7.0.200
    wget https://dot.net/v1/dotnet-install.sh -O dotnet-install.sh
    chmod +x ./dotnet-install.sh
    ./dotnet-install.sh --version 7.0.200
    rm dotnet-install.sh
    sudo apt install aspnetcore-runtime-7.0

    #Dart - v2.18.6 
    wget https://storage.googleapis.com/dart-archive/channels/stable/release/2.18.6/sdk/dartsdk-linux-x64-release.zip
    unzip dartsdk-linux-x64-release.zip
    sudo mv dart-sdk /opt/src/dart-sdk
    rm dartsdk-linux-x64-release.zip

    #Erlang - v25.2.1
    wget https://github.com/erlang/otp/releases/download/OTP-25.2.1/otp_src_25.2.1.tar.gz
    tar -zxvf otp_src_25.2.1.tar.gz 
    cd otp_src_25.2.1/
    export ERL_TOP=`pwd`
    export LANG=C 
    ./configure --prefix=/opt/erlang/25.2.1
    sudo make
    sudo make install
    cd ..
    sudo rm -r otp_src_25.2.1
    sudo rm -r otp_src_25.2.1.tar.gz 

    #Go - v1.20
    wget https://go.dev/dl/go1.20.linux-amd64.tar.gz  
    sudo mkdir /opt/src/go1.20
    sudo rm -rf /opt/src/go1.20/go && sudo tar -C /opt/src/go1.20 -xzf go1.20.linux-amd64.tar.gz
    sudo rm *.gz
    sudo apt-get install libpcre3-dev
    cd Languages/Go/regex-redux
    /opt/src/go1.20/go/bin/go mod init regex-redux
    /opt/src/go1.20/go/bin/go get github.com/GRbit/go-pcre@v1.0.0
    cd ../../..

    #Haskell -v9.4.4
    wget https://downloads.haskell.org/~ghc/9.4.4/ghc-9.4.4-x86_64-deb11-linux.tar.xz
    tar -xvf ghc-9.4.4-x86_64-deb11-linux.tar.xz
    cd ghc-9.4.4-x86_64-unknown-linux/
    ./configure --prefix=/opt/src/ghc9.4.4
    sudo make install
    cd ..
    rm -r ghc-9.4.4-x86_64-unknown-linux
    wget https://github.com/llvm/llvm-project/releases/download/llvmorg-13.0.1/llvm-13.0.1.src.tar.xz
    tar -xvf llvm-13.0.1.src.tar.xz
    sudo mv llvm-13.0.1.src /opt/src/llvm-13.0.1.src
    sudo mkdir /opt/src/llvm13.0.1/
    (cd /opt/src/llvm13.0.1/ && sudo cmake ../llvm-13.0.1.src/)
    (cd /opt/src/llvm13.0.1/ && sudo make)
    (cd /opt/src/ && sudo rm -r llvm-13.0.1.src)
    rm -r llvm-13.0.1.src.tar.xz
        # Check if the line already exists in .bashrc
        if ! grep -q 'export PATH=/opt/src/ghc9.4.4/bin:$PATH' ~/.bashrc; then
            # Add the line to .bashrc if not present
            echo 'export PATH=/opt/src/ghc9.4.4/bin:$PATH' >> ~/.bashrc
            echo "Line added to ~/.bashrc"
        else
            echo "Line already exists in ~/.bashrc"
        fi

        # Source the updated .bashrc
        source ~/.bashrc
    sudo apt install cabal-install
    cabal update
    cabal install --lib parallel
    cabal install --lib parallel-io
    cabal install --lib vector
    cabal install --lib massiv
    cabal install --lib hashable
    cabal install --lib unordered-containers
    cabal install --lib vector-algorithms
    cabal install --lib text-builder


    #Java - v20.0.2
    wget https://download.oracle.com/java/20/archive/jdk-20.0.2_linux-x64_bin.deb
    sudo dpkg -i jdk-20.0.2_linux-x64_bin.deb
    sudo apt-get install -f
    sudo update-alternatives --install /usr/bin/java java /usr/lib/jvm/jdk-20/bin/java 1
    sudo update-alternatives --install /usr/bin/javac javac /usr/lib/jvm/jdk-20/bin/javac 1
    sudo update-alternatives --install /usr/bin/jar jar /usr/lib/jvm/jdk-20/bin/jar 1
    rm jdk-20.0.2_linux-x64_bin.deb
    #-----------------Download .jar from https://jar-download.com/artifacts/it.unimi.dsi/fastutil/8.3.1/source-code
    cd Languages/Java
    unzip jar_files.zip
    sudo mkdir -p /opt/src/java-libs
    sudo mv fastutil-8.3.1.jar /opt/src/java-libs/fastutil-8.3.1.jar
    cd ../../

    #JavaScript - v19.0.1
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
    source ~/.bashrc
    nvm install 19.0.1
    nvm use 19.0.1
    npm install -g npm@8.19.2
    cd Languages/JavaScript/pidigits/
    npm install mpzjs
    cd ../../../

    #Julia - v1.8.5
    wget https://julialang-s3.julialang.org/bin/linux/x64/1.8/julia-1.8.5-linux-x86_64.tar.gz
    tar -xzvf julia-1.8.5-linux-x86_64.tar.gz
    sudo cp -r julia-1.8.5 /opt/
    sudo ln -s /opt/julia-1.8.5/bin/julia /usr/local/bin/julia
    rm -rf julia-1.8.5/ julia-1.8.5-linux-x86_64.tar.gz 

    #Lua - v5.4.4
    sudo apt-get install libpcre2-dev
    curl -R -O http://www.lua.org/ftp/lua-5.4.4.tar.gz
    tar -zxf lua-5.4.4.tar.gz
    cd lua-5.4.4
    make all test
    sudo make install
    cd ..
    rm -rf lua-5.4.4/ lua-5.4.4.tar.gz 
    wget https://luarocks.github.io/luarocks/releases/luarocks-3.9.2.tar.gz
    tar -zxpf luarocks-3.9.2.tar.gz
    cd luarocks-3.9.2/
    ./configure --lua-version=5.4 --versioned-rocks-dir
    make build
    sudo make install
    cd ..
    sudo luarocks install lrexlib-pcre2
    export LUA_CPATH="/home/diguest/Energy-Languages-PowerCap/Languages/Lua/regex-redux/luarocks-3.9.2/lua_modules/lib/lua/5.4/?.so;;$LUA_CPATH"
    sudo cp /home/diguest/Energy-Languages-PowerCap/Languages/Lua/regex-redux/luarocks-3.9.2/lua_modules/lib/lua/5.4/rex_pcre2.so /usr/lib/x86_64-linux-gnu/lua/5.4/

    #OCaml - v5.0.0
    sudo apt install opam
    opam init --disable-sandboxing --bare
    eval $(opam env)
    opam switch create 5.0.0
    eval $(opam env)
    opam install re
    opam install gmp
    opam install bigarray
    opam install ocamlfind
    opam install zarith

    #Perl - v5.36.0
    wget https://www.cpan.org/src/5.0/perl-5.36.0.tar.gz
    tar -xzf perl-5.36.0.tar.gz
    cd perl-5.36.0
    ./Configure -Dusethreads
    make
    make test
    sudo make install
    cd ../

    #PHP - v8.2.1
    wget https://www.php.net/distributions/php-8.2.1.tar.gz
    tar -xzf php-8.2.1.tar.gz
    cd php-8.2.1/
    sudo apt install -y pkg-config build-essential autoconf bison re2c \
                        libxml2-dev libsqlite3-dev libgmp-dev
    ./buildconf
    ./configure --enable-shmop --enable-pcntl --with-gmp --enable-sysvmsg
    make
    make test
    sudo make install
    cd ..
    rm -rf php-8.2.1 php-8.2.1.tar.gz
    sudo sed -i 's/;extension=shmop/extension=shmop/' /etc/php/8.1/cli/php.ini
    sudo sed -i 's/;extension=shmop/extension=shmop/' /etc/php/8.1/apache2/php.ini
    source ~/.bashrc

    #Python - v3.11.1
    (cd /usr/src && 
    sudo wget https://www.python.org/ftp/python/3.11.1/Python-3.11.1.tgz  &&
    tar -xzf Python-3.11.1.tgz &&
    cd Python-3.11.1 &&
    sudo ./configure --enable-optimizations &&
    sudo make altinstall &&
    curl -sS https://bootstrap.pypa.io/get-pip.py | python3.11 &&
    pip3.11 install gmpy2)

    #Racket - v8.7
    wget https://download.racket-lang.org/releases/8.7/installers/racket-8.7-x86_64-linux-cs.sh
    export PATH=~/bin:$PATH
    
    #Ruby - v3.2.0
    sudo apt update
    sudo apt install git curl libssl-dev libreadline-dev zlib1g-dev autoconf bison build-essential libyaml-dev libreadline-dev libncurses5-dev libffi-dev libgdbm-dev libgmp-dev
    curl -fsSL https://github.com/rbenv/rbenv-installer/raw/HEAD/bin/rbenv-installer | bash
    echo 'export PATH="$HOME/.rbenv/bin:$PATH"' >> ~/.bashrc
    echo 'eval "$(rbenv init -)"' >> ~/.bashrc
    source ~/.bashrc
    rbenv install 3.2.0
    rbenv global 3.2.0
    gem install gmp
    cd Languages/Ruby/pidigits/
    cp $(dirname $(gem which gmp))/gmp.so .
    cd ../../..

    #Rust - v1.67.0
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.bashrc
    rustup toolchain install 1.67.0
    rustup default 1.67.0
    

    #Swift - v5.8dev
    sudo apt install clang libicu-dev libxml2 git libgmp-dev
    sudo apt install binutils gnupg2 libc6-dev libcurl4 libedit2 libgcc-9-dev libsqlite3-0 libstdc++-9-dev libxml2 libz3-dev pkg-config tzdata zlib1g-dev
    wget https://download.swift.org/swift-5.8-branch/ubuntu2204/swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a/swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04.tar.gz
    tar -xzf swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04.tar.gz
    sudo mv swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04 /opt/swift
    rm -rf *.gz swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04/
    echo 'export PATH=/opt/swift/usr/bin:$PATH' >> ~/.bashrc
    source ~/.bashrc
