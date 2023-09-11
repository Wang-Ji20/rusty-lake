import * as wasm from 'scheme-frontend'
import './App.css'

function App() {
    return (
        <>
            <button onClick={() => wasm.greet()}>
                from my scheme backend
            </button>
        </>
    )
}

export default App
