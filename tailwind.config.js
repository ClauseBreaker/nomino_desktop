/**
 * Tailwind CSS Configuration
 * 
 * Defines the design system for the Nomino application including:
 * - Dark theme color palette
 * - Typography system
 * - Component styling utilities
 * - Custom shadows and effects
 */

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}',
    './src-tauri/src/**/*.rs'
  ],
  
  theme: {
    extend: {
      // Custom color palette for dark theme
      colors: {
        // Background colors
        primary: {
          50: '#f8fafc',
          100: '#f1f5f9',
          200: '#e2e8f0',
          300: '#cbd5e1',
          400: '#94a3b8',
          500: '#64748b',
          600: '#475569',
          700: '#334155',
          800: '#1e293b',
          900: '#0f172a',
          950: '#020617'
        },
        
        // Dark theme specific colors
        dark: {
          bg: '#1E1E2F',
          secondary: '#131321',
          card: '#232334',
          sidebar: '#181828',
          border: '#2A2A3E',
          hover: '#2D2D42'
        },
        
        // Accent colors for interactive elements
        accent: {
          cyan: '#00F7FF',
          'cyan-dark': '#00B2FF',
          orange: '#FF6B35',
          'orange-dark': '#E55A2B',
          yellow: '#FFD700',
          'yellow-dark': '#E6C200',
          green: '#28A745',
          'green-dark': '#1E7E34',
          red: '#FF3B30',
          'red-dark': '#E6342A'
        },
        
        // Text colors for hierarchy
        text: {
          primary: '#FFFFFF',
          secondary: '#A0A0A0',
          muted: '#6B6B6B',
          disabled: '#4A4A4A'
        }
      },
      
      // Typography system
      fontFamily: {
        sans: ['Nunito', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Consolas', 'monospace']
      },
      
      fontSize: {
        'xs': ['0.75rem', { lineHeight: '1rem' }],
        'sm': ['0.875rem', { lineHeight: '1.25rem' }],
        'base': ['1rem', { lineHeight: '1.5rem' }],
        'lg': ['1.125rem', { lineHeight: '1.75rem' }],
        'xl': ['1.25rem', { lineHeight: '1.75rem' }],
        '2xl': ['1.5rem', { lineHeight: '2rem' }],
        '3xl': ['1.875rem', { lineHeight: '2.25rem' }]
      },
      
      // Border radius system
      borderRadius: {
        'card': '16px',
        'button': '12px',
        'input': '8px',
        'badge': '6px'
      },
      
      // Shadow system for depth
      boxShadow: {
        'inner-soft': 'inset 0 2px 4px 0 rgba(0, 0, 0, 0.2)',
        'card': '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
        'card-hover': '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
        'glow-cyan': '0 0 20px rgba(0, 247, 255, 0.3)',
        'glow-orange': '0 0 20px rgba(255, 107, 53, 0.3)',
        'glow-green': '0 0 20px rgba(40, 167, 69, 0.3)'
      },
      
      // Animation system
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite'
      },
      
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' }
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' }
        }
      },
      
      // Spacing system
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
        '128': '32rem'
      }
    }
  },
  
  plugins: [
    // Add any Tailwind plugins here
  ]
}; 