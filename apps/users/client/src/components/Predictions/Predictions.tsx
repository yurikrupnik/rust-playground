import { createEffect, createSignal, For } from 'solid-js';
import axios from 'axios';
// import { User, UserDocument } from '@nx-go-playground/api/users';

// console.log('VITE_API_URLE', 'import.meta.env.VITE_API_URL');
console.log('VITE_API_URL', import.meta.env.VITE_API_URL);
console.log('VITE_AR', import.meta.env.VITE_AR);
console.log('VITE_AR', import.meta.env.PROD);
console.log('BASE_URL', import.meta.env.BASE_URL);
console.log('VITE_AR', import.meta.env.VITE_API_HOST);

function getUsers() {
  return (
    axios
      // .get<User[]>('http://localhost:8080/api/users')
      .get('http://localhost:5001/api/users')
      // .get<User[]>('http://10.96.232.73:8080/api/users')
      .then((r) => r.data)
  );
}

const Predictions = () => {
  const [data, setData] = createSignal([]);
  createEffect(() => {
    getUsers().then((res) => {
      setData(res);
    });
  });
  return (
    <div>
      <h1>Predictions</h1>
      <div>
        data here
        <For each={data()}>
          {(item: any) => {
            return <div>{item.name}</div>;
          }}
        </For>
      </div>
    </div>
  );
};

export default Predictions;
