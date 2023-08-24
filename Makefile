include $(TOPDIR)/rules.mk

PKG_NAME:=rouille_example
PKG_VERSION:=0.1.0
PKG_RELEASE:=1

PKG_BUILD_DEPENDS:=rust/host

include $(INCLUDE_DIR)/package.mk

CARGO_HOME := $(STAGING_DIR_HOST)/.cargo
RUSTFLAGS="-C linker=$(TARGET_CC_NOCACHE) -C ar=$(TARGET_AR)"

CONFIGURE_VARS += \
	CARGO_HOME=$(CARGO_HOME) \
	RUSTFLAGS=$(RUSTFLAGS)

REAL_GNU_TARGET_NAME:=arm-unknown-linux-musleabi

define Build/Compile
	cd $(PKG_BUILD_DIR) && \
          $(CONFIGURE_VARS) cargo build --release --target=$(REAL_GNU_TARGET_NAME)
endef

define Package/rouille_example
	SECTION:=examples
	CATEGORY:=Examples
	TITLE:=Rust Rouille example
endef

define Package/rouille_example/description
 Rust Rouille example
endef

define Package/rouille_example/install
	$(INSTALL_DIR) $(1)/usr/bin
	$(INSTALL_BIN) $(PKG_BUILD_DIR)/target/$(REAL_GNU_TARGET_NAME)/release/rouille_example $(1)/usr/bin
endef

$(eval $(call BuildPackage,rouille_example))
