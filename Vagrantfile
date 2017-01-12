Vagrant.configure(2) do |config|
  config.vm.box = "bento/ubuntu-16.04"
  config.vm.synced_folder ".", "/src"
   config.vm.provision "shell", inline: <<-SHELL
      sudo apt-get update
      sudo apt install -y curl git nasm qemu vim xorriso
      curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
  SHELL
end
