sudo apt-get update
pip install lazyme
sudo apt-get install make
sudo apt-get install zip unzip
sudo mkdir -p /opt/src

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

    #Dart
    wget https://storage.googleapis.com/dart-archive/channels/stable/release/2.18.6/sdk/dartsdk-linux-x64-release.zip
    unzip dartsdk-linux-x64-release.zip
    sudo mv dart-sdk /opt/src/dart-sdk
    rm -r dart-sdk
    rm dartsdk-linux-x64-release.zip

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

    #Racket
    sudo apt install racket

    #Swift
    sudo apt-get install swift
    sudo apt install clang libicu-dev libxml2 git libgmp-dev
    sudo apt install binutils gnupg2 libc6-dev libcurl4 libedit2 libgcc-9-dev libsqlite3-0 libstdc++-9-dev libxml2 libz3-dev pkg-config tzdata zlib1g-dev
    wget https://download.swift.org/swift-5.8-branch/ubuntu2204/swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a/swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04.tar.gz
    tar xzf swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04.tar.gz
    sudo mv swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04 /opt/swift
    rm -rf *.gz swift-5.8-DEVELOPMENT-SNAPSHOT-2023-03-17-a-ubuntu22.04/
    echo 'export PATH=/opt/swift/usr/bin:$PATH' >> ~/.bashrc
    source ~/.bashrc