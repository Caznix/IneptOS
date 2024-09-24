run TARGET="x86_64":
    cargo xtask run {{ TARGET }}

build TARGET="x86_64":
    cargo xtask build {{ TARGET }}

clean TARGET="all":
    cargo xtask clean {{ TARGET }}