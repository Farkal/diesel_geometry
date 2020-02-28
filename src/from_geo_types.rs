pub use crate::pg::types::geometric::{
    PgBox, PgGeometry, PgLineString, PgMultiPoint, PgMultiPolygon, PgPoint, PgPolygon,
};
use geo_types::{Coordinate, LineString, MultiPoint, MultiPolygon, Point, Polygon, Rect};

impl From<Point<f64>> for PgPoint {
    fn from(p: Point<f64>) -> Self {
        PgPoint(p.x(), p.y())
    }
}

impl From<Coordinate<f64>> for PgPoint {
    fn from(p: Coordinate<f64>) -> Self {
        let (x, y) = p.x_y();
        PgPoint(x, y)
    }
}

impl From<Rect<f64>> for PgBox {
    fn from(r: Rect<f64>) -> Self {
        PgBox(PgPoint(r.min.x, r.min.y), PgPoint(r.max.x, r.max.y))
    }
}

impl From<MultiPoint<f64>> for PgMultiPoint {
    fn from(mp: MultiPoint<f64>) -> Self {
        let points = mp.0.into_iter().map(|p| p.into()).collect();
        PgMultiPoint(points)
    }
}

impl From<LineString<f64>> for PgLineString {
    fn from(l: LineString<f64>) -> Self {
        let points = l.0.into_iter().map(|p| p.into()).collect();
        PgLineString(points)
    }
}

impl From<Polygon<f64>> for PgPolygon {
    fn from(p: Polygon<f64>) -> Self {
        let (exterior, interiors) = p.into_inner();
        let mut res: Vec<PgLineString> = interiors.into_iter().map(|p| p.into()).collect();
        res.insert(0, exterior.into());
        PgPolygon(res)
    }
}

impl From<MultiPolygon<f64>> for PgMultiPolygon {
    fn from(l: MultiPolygon<f64>) -> Self {
        let polygons = l.0.into_iter().map(|p| p.into()).collect();
        PgMultiPolygon(polygons)
    }
}
