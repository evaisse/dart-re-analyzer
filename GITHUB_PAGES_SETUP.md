# GitHub Pages Setup Instructions

This repository now has a documentation website ready to be published on GitHub Pages.

## Enable GitHub Pages

To publish the documentation website, follow these steps:

1. **Go to Repository Settings**
   - Navigate to https://github.com/evaisse/dart-re-analyzer/settings/pages
   - Or go to Settings tab → Pages (in the left sidebar)

2. **Configure Source**
   - Under "Build and deployment" → "Source"
   - Select: **Deploy from a branch**
   - Under "Branch":
     - Select branch: **main** (or master)
     - Select folder: **/docs**
   - Click **Save**

3. **Wait for Deployment**
   - GitHub will automatically build and deploy the site
   - This usually takes 1-2 minutes
   - You'll see a message: "Your site is live at https://evaisse.github.io/dart-re-analyzer/"

4. **Verify the Site**
   - Visit: https://evaisse.github.io/dart-re-analyzer/
   - You should see the documentation homepage

## 🎉 That's it!

The website will automatically update whenever changes are pushed to the `docs/` folder in the main branch.

## Local Development (Optional)

To preview the site locally before pushing:

1. Install Jekyll:
```bash
gem install bundler jekyll
```

2. Install dependencies:
```bash
cd docs
bundle install
```

3. Serve the site:
```bash
bundle exec jekyll serve
```

4. Open http://localhost:4000/dart-re-analyzer/

## Troubleshooting

### Site not showing up?
- Check the Actions tab for build errors
- Ensure the source is set to `/docs` folder
- Wait a few minutes for the first deployment

### 404 errors on pages?
- Make sure all markdown files have the `.md` extension
- Check that internal links use relative paths without `.md`
- Verify all pages have proper YAML front matter

### Styling issues?
- Clear your browser cache
- Check that `_config.yml` is valid YAML
- Ensure the theme is set correctly

## Need Help?

- GitHub Pages Documentation: https://docs.github.com/en/pages
- Jekyll Documentation: https://jekyllrb.com/docs/
