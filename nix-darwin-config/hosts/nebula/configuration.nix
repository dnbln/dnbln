{ pkgs, inputs, configurationRevision, ... }: {
  # List packages installed in system profile. To search by name, run:
  # $ nix-env -qaP | grep wget
  nixpkgs.config.allowUnfreePredicate = pkg: builtins.elem (pkgs.lib.getName pkg) [
    # "arc-browser"
    "warp-terminal"
    "signal-desktop-bin"
    "discord"
    "obsidian"
    "spotify"
  ];

  environment.systemPackages =
    [ pkgs.vim
      pkgs.neovim
      pkgs.git
      pkgs.warp-terminal
      pkgs.signal-desktop-bin
      pkgs.zotero
      pkgs.discord
      pkgs.obsidian
      pkgs.spotify
      (pkgs.coq.override {
        buildIde = true;
      })
      pkgs.coqPackages.stdlib
      pkgs.coqPackages.vscoq-language-server
      pkgs.opam
      pkgs.clang
      pkgs.gcc
      pkgs.nodejs
      pkgs.pv
      pkgs.ffmpeg
    ];



  # Necessary for using flakes on this system.
  nix.enable = false;
  nix.settings.experimental-features = "nix-command flakes";

  # Enable alternative shell support in nix-darwin.
  programs.zsh.enable = true;
  # programs.fish.enable = true;

  # Set Git commit hash for darwin-version.
  system = {
    inherit configurationRevision;
    # Used for backwards compatibility, please read the changelog before changing.
    # $ darwin-rebuild changelog
    stateVersion = 6;
    defaults = {
      dock.autohide = true;
      dock.mru-spaces = false;
      finder.AppleShowAllExtensions = true;
      finder.FXPreferredViewStyle = "clmv";
      loginwindow.LoginwindowText = "dnbln";
      screencapture.location = "~/Pictures/screenshots";
      screensaver.askForPasswordDelay = 10;
    };

    primaryUser = "dinu";
  };


  # The platform the configuration will be used on.
  nixpkgs.hostPlatform = "x86_64-darwin";

  security.pam.services.sudo_local.touchIdAuth = true;

  users.users.dinu = {
    name = "dinu";
    home = "/Users/dinu";
  };

  home-manager.users = {
    dinu = import ./home.nix;
  };
}
