sudo apt-get update
pip install lazyme
sudo apt-get install make
sudo apt-get install zip unzip
sudo mkdir -p /opt/src
sudo apt-get update
sudo apt install cmake

## Compilers ##
    #Ada
    sudo apt install gnat-11
    sudo apt-get install libgmp-dev
    sudo apt-get install libapr1-dev

    #C *precisa versao especifica*
    sudo apt install build-essential
    sudo apt-get install libpcre2-dev

    #C++ *precisa versao especifica*
    sudo apt install libtbb-dev
    sudo apt-get install libboost-all-dev

    #Chapel *compilador nao funciona*
    sudo apt-get install -y gcc g++ m4 perl python3 python3-dev bash make mawk git pkg-config cmake
    sudo apt-get install -y llvm-dev llvm clang libclang-dev libclang-cpp-dev libedit-dev
    wget https://github.com/chapel-lang/chapel/releases/download/1.29.0/chapel-1.29.0.tar.gz
    tar -xvzf chapel-1.29.0.tar.gz
    sudo rm -r chapel-1.29.0.tar.gz
    cd chapel-1.29.0/
    sudo ./configure --prefix=/opt/src/chapel-1.29.0
    sudo make
    sudo make install
    cd ..
    sudo rm -r chapel-1.29.0/

    #Csharp (.net) fasta is looking kinda weird idk
    wget https://dot.net/v1/dotnet-install.sh -O dotnet-install.sh
    chmod +x ./dotnet-install.sh
    ./dotnet-install.sh --version 7.0.200
    rm dotnet-install.sh
    sudo apt install aspnetcore-runtime-7.0
    git clone https://github.com/dotnet/corefxlab.git
    cd corefxlab/
    ~/.dotnet/dotnet restore
    cd ..

    #Dart
    wget https://storage.googleapis.com/dart-archive/channels/stable/release/2.18.6/sdk/dartsdk-linux-x64-release.zip
    unzip dartsdk-linux-x64-release.zip
    sudo mv dart-sdk /opt/src/dart-sdk
    rm -r dart-sdk
    rm dartsdk-linux-x64-release.zip

    #Intel license
    sudo apt install net-tools

    #Fortran o login nao funciona de momento

    #Erlang
    wget https://github.com/erlang/otp/releases/download/OTP-25.2.1/otp_src_25.2.1.tar.gz
    tar -zxvf otp_src_25.2.1.tar.gz 
    cd otp_src_25.2.1/
    ./configure --prefix=/opt/src/otp_src_25.2.1
    sudo make
    sudo make install
    cd ..
    rm -r otp_src_25.2.1
    rm -r otp_src_25.2.1.tar.gz 

    #GO
    wget https://go.dev/dl/go1.21.1.linux-amd64.tar.gz
    mkdir /opt/src/go1.20
    sudo rm -rf /opt/src/go1.20/go && sudo tar -C /opt/src/go1.20 -xzf go1.21.1.linux-amd64.tar.gz

    #Haskell
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
    cabal update
    cabal install --lib parallel


    #Java - o pidigits não está a dar (não consigo instalar o gmp)
    wget https://download.oracle.com/java/20/latest/jdk-20_linux-x64_bin.deb
    sudo dpkg -i jdk-20_linux-x64_bin.deb
    sudo apt-get install -f
    sudo update-alternatives --install /usr/bin/java java /usr/lib/jvm/jdk-20/bin/java 1
    sudo update-alternatives --install /usr/bin/javac javac /usr/lib/jvm/jdk-20/bin/javac 1
    sudo update-alternatives --install /usr/bin/jar jar /usr/lib/jvm/jdk-20/bin/jar 1
    #!!!!!!!!!!!!!!!!!!! É necessário sacar este jar https://jar-download.com/artifacts/it.unimi.dsi/fastutil/8.3.1/source-code
    #unzip jar_files.zip
    #sudo mkdir -p /opt/src/java-libs
    #sudo mv fastutil-8.3.1.jar /opt/src/java-libs/fastutil-8.3.1.jar

    #Racket
    sudo apt install racket

    #Swift - só o pi-digits não funciona por causa da biblioteca GMP
    sudo apt-get install swift
    sudo apt install clang libicu-dev libxml2 git libgmp-dev
    sudo apt install binutils gnupg2 libc6-dev libcurl4 libedit2 libgcc-9-dev libsqlite3-0 libstdc++-9-dev libxml2 libz3-dev pkg-config tzdata zlib1g-dev
    wget https://download.swift.org/swift-5.8-branch/ubuntu2204/swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a/swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04.tar.gz
    tar xzf swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04.tar.gz
    sudo mv swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04 /opt/swift
    rm -rf *.gz swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04/
    echo 'export PATH=/opt/swift/usr/bin:$PATH' >> ~/.bashrc
    source ~/.bashrc

    #Ruby
    sudo apt install rbenv
    sudo apt-get install -y libyaml-dev
    rbenv install 3.2.0
    rbenv global 3.2.0

    #Ocaml
    sudo apt install opam
    opam init --disable-sandboxing --bare
    eval $(opam env)
    opam switch create 5.0.0
    eval $(opam env)
    opam install re
    opam install gmp
    opam install bigarray
    opam install ocamlfind

    #Pascal - faltam mandelbrot, pidigits e binarytrees funcionarem
    sudo apt install fpc
    sudo apt-get install libpcre3-dev

    #Perl
    \curl -L https://install.perlbrew.pl | bash
    source ~/perl5/perlbrew/etc/bashrc
    perlbrew install 5.36.0

    #PHP - Versão instalada incorreta
    sudo apt update && apt upgrade -y
    sudo add-apt-repository ppa:ondrej/php
    sudo apt update
    #sudo apt install php8.2 --> não funciona
    sudo apt-get install php #v.8.1 por default

    #Python
    cd /usr/src 
    sudo wget https://www.python.org/ftp/python/3.11.1/Python-3.11.1.tgz 
    sudo tar xzf Python-3.11.1.tgz 
    cd Python-3.11.1 
    sudo ./configure --enable-optimizations 
    sudo make altinstall 
    curl -sS https://bootstrap.pypa.io/get-pip.py | python3.11 
    cd

    #Lua - pidigits não tem implementação funcional
    curl -R -O http://www.lua.org/ftp/lua-5.4.4.tar.gz
    tar zxf lua-5.4.4.tar.gz
    cd lua-5.4.4
    make all test
    sudo make install
    cd ..
    rm -rf lua-5.4.4/ lua-5.4.4.tar.gz 

    #Lisp
    wget "https://downloads.sourceforge.net/project/sbcl/sbcl/2.3.0/sbcl-2.3.0-x86-64-linux-binary.tar.bz2?ts=gAAAAABlEBakeknsJ6oJ_B5zxomXLHjCGBWYJw8S8luWp_qXTv0nvC6ESEUzgL4Io0fT3tw18PCSlBpl3hqUfz0ZWzcY-JWF2g%3D%3D&r=https%3A%2F%2Fsourceforge.net%2Fprojects%2Fsbcl%2Ffiles%2Fsbcl%2F2.3.0%2Fsbcl-2.3.0-x86-64-linux-binary.tar.bz2%2Fdownload"
    mv "sbcl-2.3.0-x86-64-linux-binary.tar.bz2?ts=gAAAAABlEBakeknsJ6oJ_B5zxomXLHjCGBWYJw8S8luWp_qXTv0nvC6ESEUzgL4Io0fT3tw18PCSlBpl3hqUfz0ZWzcY-JWF2g==&r=https:%2F%2Fsourceforge.net%2Fprojects%2Fsbcl%2Ffiles%2Fsbcl%2F2.3.0%2Fsbcl-2.3.0-x86-64-li" sbcl-2.3.0-x86-64-linux-binary.tar.bz2
    rm wget-log 
    tar -xvjf sbcl-2.3.0-x86-64-linux-binary.tar.bz2
    cd sbcl-2.3.0-x86-64-linux
    sudo sh install.sh
    cd ..
    rm -rf sbcl-2.3.0-x86-64-linux/ sbcl-2.3.0-x86-64-linux-binary.tar.bz2 
    curl -o /tmp/ql.lisp http://beta.quicklisp.org/quicklisp.lisp
    sbcl --no-sysinit --no-userinit --load /tmp/ql.lisp \
         --eval '(quicklisp-quickstart:install :path "~/.quicklisp")' \
         --eval '(ql:add-to-init-file)' \
         --quit

    #Julia
    wget https://julialang-s3.julialang.org/bin/linux/x64/1.8/julia-1.8.5-linux-x86_64.tar.gz
    tar -xzvf julia-1.8.5-linux-x86_64.tar.gz
    sudo cp -r julia-1.8.5 /opt/
    sudo ln -s /opt/julia-1.8.5/bin/julia /usr/local/bin/julia
    rm -rf julia-1.8.5/ julia-1.8.5-linux-x86_64.tar.gz 

    #JavaScript
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
    source ~/.bashrc
    nvm install 19.0.0
    nvm use 19.0.0
    npm install -g npm@latest
    cd JavaScript/pidigits/
    npm install mpzjs
    cd ../../
    
    