# SledoView Website

This directory contains the website for SledoView, a powerful console application for performing CRUD operations on SLED database files.

## Files

- `index.html` - Main website page with complete project information
- `styles.css` - CSS styling for the website with responsive design
- `script.js` - JavaScript for interactive features and animations

## Features

- **Responsive Design** - Works on desktop, tablet, and mobile devices
- **Interactive Terminal Demo** - Cycling examples of SledoView in action
- **Copy-to-Clipboard** - Easy copying of installation commands
- **Smooth Animations** - Fade-in effects and smooth scrolling
- **Modern Design** - Clean, professional appearance with gradients and shadows

## Hosting

This is a static website that can be hosted on any web server or static hosting service like:

- GitHub Pages
- Netlify
- Vercel
- Any traditional web hosting

## Local Development

To view the website locally:

1. Open `index.html` in a web browser directly, or
2. Serve with a local HTTP server:
   ```bash
   # Using Python 3
   python -m http.server 8000
   
   # Using Node.js (if you have http-server installed)
   npx http-server
   
   # Using PHP
   php -S localhost:8000
   ```

Then navigate to `http://localhost:8000` in your browser.

## Dependencies

The website uses the following external dependencies via CDN:

- **Google Fonts** - Inter font family for modern typography
- **Highlight.js** - Syntax highlighting for code examples
- **GitHub Dark Theme** - Syntax highlighting theme

All dependencies are loaded from CDN, so no local installation is required.

## Customization

To customize the website:

1. **Colors** - Modify the CSS gradient colors and theme colors in `styles.css`
2. **Content** - Update text, examples, and links in `index.html`
3. **Terminal Demo** - Modify the demo content in `script.js` (cycleDemos function)
4. **Fonts** - Change the Google Fonts import in `index.html` and CSS font-family

## Browser Support

The website supports all modern browsers including:

- Chrome 60+
- Firefox 55+
- Safari 12+
- Edge 79+

## Performance

The website is optimized for performance with:

- Minimal external dependencies
- Optimized images (CSS gradients instead of image backgrounds)
- Efficient CSS and JavaScript
- Responsive images and layouts
