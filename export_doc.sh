if [ "$TRAVIS_BRANCH" != 'master' ]; then
    echo "skipping doc export because branch is not master"
    exit;
fi

if [ "$DOC" != "true" ]; then
    echo "skipping doc export because not enabled for this build"
    exit;
fi

echo "exporting docs"

export PATH=$HOME/.local/bin:$PATH
cargo doc
echo "<meta http-equiv=refresh content=0;url=${CRATE}/index.html>" > target/doc/index.html
pip install ghp-import --user `whoami`
ghp-import -n target/doc
git push -qf https://${TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
