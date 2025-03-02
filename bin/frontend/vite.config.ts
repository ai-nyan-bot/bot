import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    build: {
        outDir: './build',
        sourcemap: true
    },
    resolve: {
        alias: {
            '@app': '/src',
            '@components': '/src/components',
            '@hooks': '/src/hooks',
            '@modals': '/src/modals',
            '@pages': '/src/pages',
            '@states': '/src/states',
            '@styles': '/src/styles',
            '@types': '/src/types',
            '@utils': '/src/utils',
        },
    },
    server: {
        port: 5173,
        open: true
    }
})