MAKE_FLAGS := -j8
WGET := wget
WGET_FLAGS := -O -
TAR := tar

# Version of the various tools we'll be using
BINUTILS_VERSION := 2.27
MIRROR := https://ftp.gnu.org/gnu

BINUTILS_MIRROR := $(MIRROR)/binutils/binutils-$(BINUTILS_VERSION).tar.bz2
BINUTILS_SRC = binutils-$(BINUTILS_VERSION)
BINUTILS_CONFIG = $(BINUTILS_SRC)/configure

#########################
# PlayStation toolchain #
#########################

PSX_TARGET = mipsel-unknown-elf

PSX_AS  = $(PSX_TARGET)-as

all: $(PSX_AS)

PSX_BINUTILS_BUILD = build

PSX_BINUTILS_FLAGS = --target=$(PSX_TARGET) \
                      --with-float=soft --enable-softfloat

$(PSX_AS): $(BINUTILS_CONFIG)
	mkdir -p $(PSX_BINUTILS_BUILD)
	cd $(PSX_BINUTILS_BUILD) && \
	../$(BINUTILS_CONFIG) $(PSX_BINUTILS_FLAGS) && \
	$(MAKE) $(MAKE_FLAGS)
	cp $(PSX_BINUTILS_BUILD)/ld/ld-new ld
	cp $(PSX_BINUTILS_BUILD)/gas/as-new as
	cp $(PSX_BINUTILS_BUILD)/binutils/ar ar

###################
# Source download #
###################

$(BINUTILS_CONFIG):
	mkdir -p $(BINUTILS_SRC)
	$(WGET) $(WGET_FLAGS) $(BINUTILS_MIRROR) | \
	$(TAR) -C $(BINUTILS_SRC) --strip-components=1 -xj
