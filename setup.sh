SETUP_DIR=/opt/autowall

# Build autowall
cargo build --manifest-path Cargo.toml 2>&1 &> build.log

# Create Required Directories And Files
echo "Creating Directories.."
sudo mkdir -p $SETUP_DIR/wallpapers
touch $SETUP_DIR/autowall.log

# Copy autowall executable, environment and scripts
echo "Copying Required Files and Directories.."
cp -f -r target/debug/autowall autowall.env scripts/ $SETUP_DIR/

# Grant permissions
echo "Setting Up Permissions.."
sudo chown -R nkitan:nkitan $SETUP_DIR
sudo chmod +x $SETUP_DIR/autowall

echo "Setup is complete"