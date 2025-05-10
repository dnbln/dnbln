{config, pkgs, ...}: {
  nixpkgs.config.allowUnfreePredicate = pkg: builtins.elem (pkgs.lib.getName pkg) [
    "arc-browser"
    "spotify"
    "discord"
    "obsidian"
    "warp-terminal"
  ];
  home.packages = [
    pkgs.arc-browser
    pkgs.spotify
    pkgs.discord
    pkgs.zotero
    pkgs.podman
    pkgs.obsidian
    pkgs.warp-terminal
    pkgs.alire
  ];
  programs.zsh.enable = true;
  home.stateVersion = "25.05";

  programs.home-manager.enable = true;
  programs.git = {
    enable = true;
    package = pkgs.gitFull;
    userName = "Dinu Blanovschi";
    userEmail = "git@dnbln.dev";

    signing = {
      format = "ssh";
      signByDefault = true;
    };

    extraConfig = {
      credential.helper = "osxkeychain";

      init.defaultbranch = "trunk";
    };
  };
}
