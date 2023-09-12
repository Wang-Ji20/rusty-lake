import * as wasm from 'scheme-frontend'
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import {faPaperPlane} from '@fortawesome/free-solid-svg-icons'
import './App.css'

function InputScheme() {
    return (
        <div>
            <input type="text" />
            <button>
                <FontAwesomeIcon icon={faPaperPlane} />
            </button>
        </div>
    )
}

function SchemeResult(expr: string, result: string) {
    return (
        <div>
            <p>{'uinput ]=>  '}{expr}</p>
            <p>{'answer]=>     '}{result}</p>
        </div>
    )
}

function App() {
    return (
        <>
            {InputScheme()}
            {SchemeResult('(+ 1 2)', '3')}
            <button onClick={() => wasm.greet()}>
                from my scheme backend
            </button>
        </>
    )
}

export default App
