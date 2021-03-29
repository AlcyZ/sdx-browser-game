import { mat4 } from 'gl-matrix';

export default class Camera {
    private readonly _position: mat4;

    constructor(position: mat4) {
        this._position = position;
    }

    public static new(): Camera {
        return new Camera(mat4.create());
    }

    public position(): mat4 {
        return this._position;
    }

    public rotateX(radian: number): void {
        mat4.rotateX(this._position, this._position, radian);
    }

    public rotateY(radian: number): void {
        mat4.rotateX(this._position, this._position, radian);
    }

    public rotateZ(radian: number): void {
        mat4.rotateX(this._position, this._position, radian);
    }

    public translate(x: number, y: number, z: number): void {
        mat4.translate(this._position, this._position, [x, -y, z]);
    }
}