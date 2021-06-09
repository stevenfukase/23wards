import '../styles/App.css';
import postalData from '../tokyo.json'

export default function App() {
  return (
    <div className="App">
      {JSON.stringify(postalData)}
    </div>
  );
}
