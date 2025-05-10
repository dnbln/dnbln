{config, pkgs, ...}: {
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
