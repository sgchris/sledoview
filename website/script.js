// Copy to clipboard functionality
function copyToClipboard(button) {
    const codeBlock = button.previousElementSibling.querySelector('code');
    const text = codeBlock.textContent;
    
    navigator.clipboard.writeText(text).then(function() {
        // Store original text
        const originalText = button.textContent;
        
        // Change button text temporarily
        button.textContent = '✓ Copied!';
        button.style.background = 'rgba(76, 175, 80, 0.3)';
        
        // Reset after 2 seconds
        setTimeout(function() {
            button.textContent = originalText;
            button.style.background = 'rgba(255, 255, 255, 0.1)';
        }, 2000);
    }).catch(function(err) {
        console.error('Could not copy text: ', err);
        
        // Fallback for older browsers
        const textArea = document.createElement('textarea');
        textArea.value = text;
        document.body.appendChild(textArea);
        textArea.focus();
        textArea.select();
        
        try {
            document.execCommand('copy');
            button.textContent = '✓ Copied!';
            button.style.background = 'rgba(76, 175, 80, 0.3)';
            
            setTimeout(function() {
                button.textContent = '📋';
                button.style.background = 'rgba(255, 255, 255, 0.1)';
            }, 2000);
        } catch (err) {
            console.error('Fallback: Could not copy text');
        }
        
        document.body.removeChild(textArea);
    });
}

// Smooth scrolling for anchor links
document.addEventListener('DOMContentLoaded', function() {
    // Add smooth scrolling to all anchor links
    const links = document.querySelectorAll('a[href^="#"]');
    
    links.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            const targetId = this.getAttribute('href');
            const targetSection = document.querySelector(targetId);
            
            if (targetSection) {
                targetSection.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
});

// Terminal typing animation effect
document.addEventListener('DOMContentLoaded', function() {
    const typingElements = document.querySelectorAll('.typing-text');
    
    typingElements.forEach(element => {
        const text = element.textContent;
        const speed = 100; // typing speed in milliseconds
        
        // Clear the text initially
        element.textContent = '';
        element.style.borderRight = '2px solid #ffffff';
        
        let i = 0;
        function typeWriter() {
            if (i < text.length) {
                element.textContent += text.charAt(i);
                i++;
                setTimeout(typeWriter, speed);
            } else {
                // Remove cursor after typing is complete
                setTimeout(() => {
                    element.style.borderRight = 'none';
                }, 1000);
            }
        }
        
        // Start typing after a delay
        setTimeout(typeWriter, 2000);
    });
});

// Intersection Observer for animations
document.addEventListener('DOMContentLoaded', function() {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver(function(entries) {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, observerOptions);
    
    // Observe elements for fade-in animation
    const animatedElements = document.querySelectorAll('.feature-card, .command-card, .contribute-card');
    
    animatedElements.forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(30px)';
        el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
        observer.observe(el);
    });
});

// Add some interactive feedback for buttons
document.addEventListener('DOMContentLoaded', function() {
    const buttons = document.querySelectorAll('.btn');
    
    buttons.forEach(button => {
        button.addEventListener('mouseenter', function() {
            this.style.transform = 'translateY(-2px)';
        });
        
        button.addEventListener('mouseleave', function() {
            this.style.transform = 'translateY(0)';
        });
        
        button.addEventListener('mousedown', function() {
            this.style.transform = 'translateY(0)';
        });
        
        button.addEventListener('mouseup', function() {
            this.style.transform = 'translateY(-2px)';
        });
    });
});

// Terminal demo cycling effect
document.addEventListener('DOMContentLoaded', function() {
    const terminalContent = document.querySelector('.terminal-content');
    
    if (terminalContent) {
        const demos = [
            `$ sledoview my-database.db

<span class="text-cyan">SledoView - SLED Database Viewer</span>
<span class="text-cyan">═══════════════════════════════════</span>
<span class="text-green">✓ Database validation passed</span>
<span class="text-green">✓ Successfully opened database: my-database.db</span>
<span class="text-green">✓ Database is writable - modification commands available</span>

<span class="text-blue">Interactive SLED Database Viewer</span>
Type 'help' for available commands or 'exit' to quit.

<span class="text-gray">> </span>set user_123 "John Doe"
<span class="text-green">✓ Successfully set key 'user_123'</span>

<span class="text-gray">> </span>get user_123
<span class="text-cyan">Key: user_123</span>
<span class="text-cyan">Size: 8 bytes</span>
<span class="text-cyan">UTF-8: Yes</span>
<span class="text-cyan">Value:</span>
<span class="text-white">John Doe</span>

<span class="text-gray">> </span>list user_*
<span class="text-yellow">Found 3 keys:</span>
<span class="text-white">  1: user_001</span>
<span class="text-white">  2: user_123</span>
<span class="text-white">  3: user_admin</span>`,

            `$ sledoview config.db

<span class="text-cyan">SledoView - SLED Database Viewer</span>
<span class="text-cyan">═══════════════════════════════════</span>
<span class="text-green">✓ Database validation passed</span>
<span class="text-green">✓ Successfully opened database: config.db</span>

<span class="text-gray">> </span>count
<span class="text-yellow">Total records: 24</span>

<span class="text-gray">> </span>search *@gmail.com
<span class="text-yellow">Found 2 matches:</span>
<span class="text-white">  email_john => john.doe@gmail.com</span>
<span class="text-white">  contact => support@gmail.com</span>

<span class="text-gray">> </span>set "app settings" "theme=dark;lang=en"
<span class="text-green">✓ Successfully set key 'app settings'</span>

<span class="text-gray">> </span>delete temp_data
<span class="text-green">✓ Successfully deleted key 'temp_data'</span>`,

            `$ sledoview logs.db

<span class="text-cyan">SledoView - SLED Database Viewer</span>
<span class="text-cyan">═══════════════════════════════════</span>
<span class="text-green">✓ Database validation passed</span>
<span class="text-green">✓ Successfully opened database: logs.db</span>

<span class="text-gray">> </span>list regex log_\\d{4}
<span class="text-yellow">Found 12 keys:</span>
<span class="text-white">  1: log_2025</span>
<span class="text-white">  2: log_2024</span>
<span class="text-white">  3: log_2023</span>

<span class="text-gray">> </span>get log_2025
<span class="text-cyan">Key: log_2025</span>
<span class="text-cyan">Size: 1,024 bytes</span>
<span class="text-cyan">UTF-8: Yes</span>
<span class="text-cyan">Value:</span>
<span class="text-white">{"level": "info", "timestamp": "2025-08-21"}</span>`
        ];
        
        let currentDemo = 0;
        
        function cycleDemos() {
            terminalContent.innerHTML = demos[currentDemo];
            currentDemo = (currentDemo + 1) % demos.length;
        }
        
        // Change demo every 8 seconds
        setInterval(cycleDemos, 8000);
    }
});

// Add GitHub stars count (optional - requires API call)
document.addEventListener('DOMContentLoaded', function() {
    // This would typically fetch from GitHub API
    // For now, we'll just add a placeholder
    const githubLinks = document.querySelectorAll('a[href*="github.com/sgchris/sledoview"]');
    
    githubLinks.forEach(link => {
        // You could add star count here if you want to make an API call
        // link.innerHTML += ' ⭐ (GitHub)';
    });
});
